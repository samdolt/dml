extern crate dml;
extern crate rustc_serialize;
extern crate docopt;

#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;
use std::env;

use dml::Command;

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

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_version: bool,
}


fn main() {

    // In Docopt, option_first meen all option (--opt -o) after the
    // first positional argument are interpreted as positional
    //
    // -> dml --verbose build -o html
    // --verbose is a option and -o html are arg, stored in arg_args vector
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.options_first(true).decode()
                            })
                            .unwrap_or_else(|e| e.exit());

    env_logger::init().unwrap();

    match args.arg_command {
        Some(cmd)   => cmd.run(&env::args().collect()),
        None        => {
            if args.flag_version {
                print_info();
            } else {
                unreachable!("Somethings bad here");
            }
        }
    }
}

fn print_info() {
    println!("Dml version {}", env!("CARGO_PKG_VERSION"));
}
