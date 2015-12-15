extern crate dml;
extern crate rustc_serialize;
extern crate docopt;
extern crate subcmd;

use subcmd::Handler;
use dml::command::get_all_commands;

#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
dml - Dolt Markup Language

Usage:
  dml <command> [<args>...]
  dml -h | -v

Options:
  -h --help                 Show this screen.
  -v --version              Show version

Some common dml commands are:
  build     Compile the current project
  clean     Remove the target directory
  new       Create a new dml project
  process   Process a single file

See 'dml help <command>' for more information on a specific command.
";


fn main() {
    env_logger::init().unwrap();

    let cmds = get_all_commands();

    let mut handler = Handler::new();

    for cmd in cmds {
        handler.add(cmd);
    }

    handler.run();
}

fn print_info() {
    println!("Dml version {}", env!("CARGO_PKG_VERSION"));
}
