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

/// Available command on the command line
#[derive(Debug, RustcDecodable)]
pub enum Command {
    Build, New, Clean, Process,
}

impl Command {

    /// Run the specified command with given argv
    pub fn run(self, args: &Vec<String>) {
        match self {
            Command::Build      =>  println!("Build"),
            Command::New        =>  println!("New"),
            Command::Clean      =>  println!("Clean"),
            Command::Process    =>  ProcessCommand::run(&args),
        }
    }
}

trait DmlCommand {
    fn run(argv: &Vec<String>);
}
