mod client;
mod conf;
mod opts;

use anyhow::{anyhow, Result};
use client::YoutrackService;
pub use conf::Configuration;
pub use opts::Opts;

#[derive(Debug)]
pub struct Command {
    pub issue_id: String,
    pub query: String,
    pub comment: Option<String>,
}

type ParsedCommand = String;

pub trait CommandService {
    fn execute_command<'a>(&self, command: Command) -> Result<Vec<ParsedCommand>>;
}

pub fn run(opts: Opts, conf: Configuration) -> Result<()> {
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
            let command_service = YoutrackService::new(&conf.youtrack_url, &conf.auth_token);

            let executed_commands = command_service.execute_command(command)?;
            println!("Executed commands:");
            executed_commands
                .iter()
                .for_each(|command| println!("    {}", command));
            Ok(())
        }
        Opts::Console => Err(anyhow!("Console mode not implemented")),
    }
}
