extern crate dml;
extern crate rustc_serialize;
extern crate docopt;
extern crate subcmd;

use subcmd::CmdHandler;
use subcmd::CmdResult;
use dml::command::get_all_commands;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;


fn main() {
    env_logger::init().unwrap();

    let cmds = get_all_commands();

    let mut handler = CmdHandler::new();
    handler.set_description("Dolt Markup Language");

    for cmd in cmds {
        handler.add(cmd);
    }

    match handler.run() {
        CmdResult::Help(msg)           => msg.print(),
        CmdResult::HelpForCmd(cmd)     => cmd.print_help(),
        CmdResult::BadUsage(msg)       => msg.print(),
        CmdResult::UnknowCmd(msg)      => msg.print(),
        CmdResult::Cmd(cmd)            => cmd.run(),
    }
}

fn print_info() {
    println!("Dml version {}", env!("CARGO_PKG_VERSION"));
}
