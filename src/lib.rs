mod client;
mod conf;
mod opts;

use anyhow::{anyhow, Result};
use client::YoutrackService;
pub use conf::Configuration;
pub use opts::Opts;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

#[derive(Debug)]
pub struct Command {
    pub issue_id: String,
    pub query: String,
    pub comment: Option<String>,
}

type ParsedCommand = String;

pub trait CommandService {
    fn execute_command(&self, command: Command) -> Result<Vec<ParsedCommand>>;
}

#[derive(Debug)]
pub struct ListedIssue {
    pub id: String,
    pub summary: String,
}

pub trait ListingService {
    fn list_issues(&self, query: String, limit: Option<u8>, offset: u8)
        -> Result<Vec<ListedIssue>>;
}

pub fn run(opts: Opts, conf: Configuration) -> Result<()> {
    let youtrack_service = YoutrackService::new(&conf.youtrack_url, &conf.auth_token);

    match opts {
        Opts::Issue {
            issue_id,
            command,
            comment,
        } => {
            let command = Command {
                issue_id,
                query: command,
                comment,
            };

            let executed_commands = youtrack_service.execute_command(command)?;
            println!("Executed commands:");
            executed_commands
                .iter()
                .for_each(|command| println!("    {}", command));
            Ok(())
        }
        Opts::List {
            query,
            limit,
            offset,
        } => {
            let issues = youtrack_service.list_issues(query, limit, offset.unwrap_or(0))?;
            let mut grid = Grid::new(GridOptions {
                filling: Filling::Spaces(2),
                direction: Direction::LeftToRight,
            });

            for issue in issues {
                grid.add(Cell::from(issue.id));
                grid.add(Cell::from(issue.summary));
            }

            print!("{}", grid.fit_into_columns(2));
            Ok(())
        }
        Opts::Console => Err(anyhow!("Console mode not implemented")),
    }
}
