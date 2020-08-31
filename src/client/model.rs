use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList {
    pub query: String,
    pub caret: i32,
    pub comment: Option<String>,
    pub issues: Vec<Issue>,
    pub commands: Option<Vec<ParseCommand>>,
    pub silent: bool,
    pub suggestions: Option<Vec<Suggestion>>,
    #[serde(rename = "usesMarkdown")]
    pub uses_markdown: bool,
}

impl Default for CommandList {
    fn default() -> Self {
        CommandList {
            query: "".to_string(),
            caret: 0,
            comment: None,
            issues: Vec::new(),
            commands: None,
            silent: false,
            suggestions: None,
            uses_markdown: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Issue {
    #[serde(rename = "idReadable")]
    pub id_readable: String,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseCommand {
    pub id: String,
    pub description: String,
    pub delete: bool,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suggestion {
    pub option: String,
    pub caret: i32,
    pub description: String,
    #[serde(rename = "completionStart")]
    pub completion_start: i32,
    #[serde(rename = "completionEnd")]
    pub completion_end: i32,
    #[serde(rename = "matchingStart")]
    pub matching_start: i32,
    #[serde(rename = "matchingEnd")]
    pub matching_end: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
    pub error_destription: Option<String>,
}
