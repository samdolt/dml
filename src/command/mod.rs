//! The `command` module contains helper function for performing interactive
//! action.
//!
//! # Warning
//!
//! This function use stdin and stdout to communicate with a user.
//!
//! If you want to handle i/o and user interface, use functions of the
//! `dml::helpers` module.

use subcmd::Command;

mod process;
use self::process::ProcessCommand;

mod clean;
use self::clean::CleanCommand;

mod new;
use self::new::NewCommand;

mod build;
use self::build::BuildCommand;

pub fn get_all_commands() -> Vec<Box<Command>> {
    let mut cmds: Vec<Box<Command>> = Vec::new();

    cmds.push(Box::new(ProcessCommand));
    cmds.push(Box::new(CleanCommand));
    cmds.push(Box::new(NewCommand));
    cmds.push(Box::new(BuildCommand));

    cmds
}
