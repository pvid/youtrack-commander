use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Youtrack Commander",
    about = "Execute commands on Youtrack issues from the command line"
)]
pub enum Opts {
    #[structopt(about = "Single command mode")]
    Issue {
        issue_id: String,
        command: String,
        #[structopt(short = "k", long = "comment")]
        comment: Option<String>,
    },
    #[structopt(about = "Interactive console mode (to be implemented)")]
    Console,
}
