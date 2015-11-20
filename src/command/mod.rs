//! The `command` module contains helper function for performing interactive
//! action.
//!
//! # Warning
//!
//! This function use stdin and stdout to communicate with a user.
//!
//! If you want to handle i/o and user interface, use functions of the
//! `dml::helpers` module.

mod process;
use self::process::ProcessCommand;

mod help;
use self::help::HelpCommand;

mod clean;
use self::clean::CleanCommand;

mod new;
use self::new::NewCommand;

/// Available command on the command line
#[derive(Debug, RustcDecodable)]
pub enum Command {
    Build, New, Clean, Process, Help,
}

impl Command {

    /// Run the specified command with given argv
    pub fn run(self, args: &Vec<String>) {
        match self {
            Command::Build      =>  println!("Build"),
            Command::New        =>  NewCommand::run(&args),
            Command::Clean      =>  CleanCommand::run(&args),
            Command::Process    =>  ProcessCommand::run(&args),
            Command::Help       =>  HelpCommand::run(&args),
        }
    }

    pub fn help(self) -> String {
        match self {
            Command::Process    => ProcessCommand::help(),
            Command::Help       => HelpCommand::help(),
            Command::Clean      => CleanCommand::help(),
            Command::New        => NewCommand::help(),
            _                   => "--".to_string(),
        }
    }
}

trait DmlCommand {
    fn run(argv: &Vec<String>);
    fn help() -> String;
}
