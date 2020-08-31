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
    #[structopt(about = "List issues using a search query")]
    List {
        query: String,
        #[structopt(short = "l", long = "limit")]
        limit: Option<u8>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u8>,
    },
    #[structopt(about = "Interactive console mode (to be implemented)")]
    Console,
}
