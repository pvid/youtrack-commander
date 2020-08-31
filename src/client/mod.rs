mod model;

use self::model::*;
use crate::{Command, CommandService, ListedIssue, ListingService, ParsedCommand};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use serde::de::DeserializeOwned;

pub struct YoutrackService<'a> {
    youtrack_url: &'a str,
    auth_token: &'a str,
    client: Client,
}

impl YoutrackService<'_> {
    pub fn new<'a>(youtrack_url: &'a str, auth_token: &'a str) -> YoutrackService<'a> {
        YoutrackService {
            youtrack_url,
            auth_token,
            client: Client::new(),
        }
    }

    fn commands(&self, command_list: &CommandList) -> Result<CommandList> {
        let request_url = format!("{}/api/commands", self.youtrack_url);

        let request = self
            .client
            .post(&request_url)
            .bearer_auth(self.auth_token)
            .query(&[("fields", "query,caret,comment,issues(idReadable),commands(id,description,delete,error),silent,suggestions(completionStart,completionEnd,matchingStart,matchingEnd,caret,option,description),usesMarkdown")])
            .json(command_list);
        let response = request.send()?;
        handle_response(response)
    }

    fn list_issues(&self, query: String, limit: Option<u8>, offset: u8) -> Result<Vec<Issue>> {
        let request_url = format!("{}/api/issues", self.youtrack_url);

        let offset_string = offset.to_string();
        let limit_string = limit.map(|x| x.to_string()).unwrap_or(String::from(""));

        let mut headers = vec![
            ("fields", "idReadable,summary"),
            ("$skip", &offset_string),
            ("query", &query),
        ];

        if limit.is_some() {
            headers.push(("$top", &limit_string))
        }

        let request = self
            .client
            .get(&request_url)
            .bearer_auth(self.auth_token)
            .query(&headers);

        let response = request.send()?;
        handle_response(response)
    }
}

fn handle_response<D: DeserializeOwned>(response: Response) -> Result<D> {
    if response.status().is_success() {
        let response_body: D = response
            .json()
            .context("Failed to parse API response body")?;
        Ok(response_body)
    } else {
        let status = response.status();
        let error_body: Error = response.json().context(format!(
            "Response has status code {}, but failed to parse response body",
            status
        ))?;
        Err(anyhow!(
            "Status code: {}, error_body: {:#?}",
            status,
            error_body
        ))
    }
}

impl CommandService for YoutrackService<'_> {
    fn execute_command(&self, command: Command) -> Result<Vec<ParsedCommand>> {
        let caret = command.query.chars().count() - 1;
        let command_list = CommandList {
            query: command.query,
            caret: caret as i32,
            comment: command.comment,
            issues: vec![Issue {
                id_readable: command.issue_id.to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        self.commands(&command_list).map(|command_list| {
            command_list
                .commands
                .unwrap()
                .into_iter()
                .map(|command| remove_html_tag(&command.description))
                .collect()
        })
    }
}

lazy_static! {
    static ref TAG_REGEX: Regex = Regex::new("<[^>]+>").unwrap();
}

fn remove_html_tag(input: &str) -> String {
    TAG_REGEX.replace_all(input, "").into_owned()
}

impl ListingService for YoutrackService<'_> {
    fn list_issues(
        &self,
        query: String,
        limit: Option<u8>,
        offset: u8,
    ) -> Result<Vec<ListedIssue>> {
        let issues = self.list_issues(query, limit, offset)?;
        let result = issues
            .into_iter()
            .map(|issue| ListedIssue {
                id: issue.id_readable,
                summary: issue.summary.unwrap(),
            })
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::remove_html_tag;

    #[test]
    fn remove_tag_span() {
        let input = "Assigned <span class=\\\"bold\\\">john.doe</span>";
        let expected = "Assigned john.doe";
        assert_eq!(remove_html_tag(input), expected);
    }
}
