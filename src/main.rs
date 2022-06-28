// have seen the npm install work on windows
// now trying for Mac OS

mod actions;
mod types;

use crate::types::Crust;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    command: String,
    arg: Option<String>,
}

fn main() {
    Crust::new(None);
    Crust::run_cmd(Cli::from_args());
}
