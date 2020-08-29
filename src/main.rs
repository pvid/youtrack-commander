use anyhow::Result;
use config::{Config, File};
use std::path::Path;
use structopt::StructOpt;
use youtrack_commander::{run, Configuration, Opts};

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let conf_path = {
        let mut path = dirs::home_dir().unwrap();
        path.push(Path::new(".youtrack/commander.yml"));
        path
    };
    let mut conf = Config::default();
    conf.merge(File::from(conf_path))?;
    let typed_configuration = conf.try_into::<Configuration>()?;

    run(opts, typed_configuration)
}
