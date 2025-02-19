use crate::actions::{
    add_commit_push, checkout_new, get_status, log_commits, reset_branch, show_help,
};
use crate::types::Actions::{Acp, Cob, Help, Log, SoftReset, Status};
use crate::Cli;

pub struct CrustConfig {
    pub verbosity: Option<u8>,
    pub hide: Option<bool>,
    pub dump_location: Option<String>,
    pub ee_img_path: Option<String>,
}

pub struct Crust {
    pub config: CrustConfig,
}

impl Crust {
    pub fn new(crust_fig: Option<CrustConfig>) -> Self {
        let default_config = CrustConfig {
            verbosity: Some(2),
            hide: Some(false),
            dump_location: Some(String::from("./")),
            ee_img_path: None,
        };
        let config = match crust_fig {
            Some(config) => config,
            None => default_config,
        };

        Crust { config }
    }
}

impl Crust {
    pub fn run_cmd(args: Cli) {
        let sub_cmd = args.arg.unwrap_or_else(|| "".to_string());
        let output = match args.command {
            x if x == Acp.value() || x == Acp.short_value() => Acp.method(sub_cmd),
            x if x == Cob.value() || x == Cob.short_value() => Cob.method(sub_cmd),
            x if x == Help.value() || x == Help.short_value() => Help.method(sub_cmd),
            x if x == Log.value() || x == Log.short_value() => Log.method(sub_cmd),
            x if x == SoftReset.value() || x == SoftReset.short_value() => {
                SoftReset.method(sub_cmd)
            }
            x if x == Status.value() || x == Status.short_value() => Status.method(sub_cmd),
            x if x == "*" => display_msg(),
            _ => Help.method(sub_cmd),
        };
        println!("{}", output);
    }
}

fn display_msg() -> String {
    String::from(" Welcome to Crust \u{1F35E} \nTry typing `crust help`")
}

pub enum Actions {
    Acp,
    Cob,
    Help,
    Log,
    Status,
    SoftReset,
}

impl Actions {
    pub fn value(&self) -> String {
        match *self {
            Actions::Acp => String::from("done"),
            Actions::Cob => String::from("cob"),
            Actions::Help => String::from("help"),
            Actions::Log => String::from("log"),
            Actions::SoftReset => String::from("soft"),
            Actions::Status => String::from("st"),
        }
    }
}

impl Actions {
    pub fn method(&self, sub_cmd: String) -> String {
        match *self {
            Actions::Acp => add_commit_push(sub_cmd),
            Actions::Cob => checkout_new(sub_cmd),
            Actions::Help => show_help(),
            Actions::Log => log_commits(sub_cmd),
            Actions::SoftReset => reset_branch(sub_cmd),
            Actions::Status => get_status(),
        }
    }
}

impl Actions {
    pub fn short_value(&self) -> String {
        match *self {
            Actions::Acp => String::from("acp"),
            Actions::Cob => String::from("nb"),
            Actions::Help => String::from("h"),
            Actions::Log => String::from("l"),
            Actions::SoftReset => String::from("sr"),
            Actions::Status => String::from("s"),
        }
    }
}

pub enum RootCmd {
    Git,
    Grep,
}

impl RootCmd {
    pub fn value(&self) -> String {
        match *self {
            RootCmd::Git => String::from("git"),
            RootCmd::Grep => String::from("grep"),
        }
    }
}

// #[derive(StructOpt)]
pub enum GitCommands {
    Add,
    Branch,
    Checkout,
    Commit,
    Log,
    Push,
    Status,
    Reset,
    // Pull,
    // Stash,
    // Revert,
    // Pop,
    // Apply,
    // Hard,
    // Soft,
}

impl GitCommands {
    pub fn value(&self) -> String {
        match *self {
            GitCommands::Add => String::from("add"),
            GitCommands::Branch => String::from("branch"),
            GitCommands::Checkout => String::from("checkout"),
            GitCommands::Commit => String::from("commit"),
            GitCommands::Log => String::from("log"),
            GitCommands::Push => String::from("push"),
            GitCommands::Status => String::from("status"),
            GitCommands::Reset => String::from("reset"),
            // GitCommands::Pull => String::from("pull"),
            // GitCommands::Stash => String::from("stash"),
            // GitCommands::Revert => String::from("revert"),
            // GitCommands::Pop => String::from("pop"),
            // GitCommands::Apply => String::from("apply"),
            // GitCommands::Hard => String::from("hard"),
            // GitCommands::Soft => String::from("soft"),
        }
    }
}

pub struct HelpInfo {
    pub descriptions: Vec<String>,
    pub commands: Vec<String>,
}

impl HelpInfo {
    pub fn display(&self) -> String {
        println!("\n\u{1F419}   Welcome to crust");
        println!("     v0.0.1");
        let mut table = vec![];
        let mut row: String;
        for (i, cmd) in self.commands.iter().enumerate() {
            row = format!(
                "\u{1F680}   {0: <10}   \u{1F9AE}   {1: <10}\n",
                &self.descriptions[i], cmd
            );
            table.push(row);
        }
        println!(
            "\n{0: <10}   {1: <10}\n",
            "\u{1F680}   Description          ", "\u{1F9AE}   Command     "
        );
        table.join("")
    }
}
