extern crate dml;
extern crate rustc_serialize;
extern crate docopt;

#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;
use std::io::stdout;
use std::io::Write;
use std::fs::File;
use std::env;

use dml::process;
use dml::Format;
use dml::Config;
use dml::Command;
use dml::command::run;

const USAGE: &'static str = "
DML - Dolt Markup Language

Usage:
  dml <command> [<args>...]
  dml [options]

Options:
  -h --help                 Show this screen.
  -v --version              Show version

Some common DML commands are:
  build     Compile the current project
  clean     Remove the target directory
  new       Create a new dml project
  process   Process a single file

See 'cargo help <command>' for more information on a specific command.
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
        Some(cmd)   => run(cmd, &env::args().collect()),
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


fn project_mode(args: &Args) {
    unimplemented!()
}

fn single_file_mode(args: &Args) {


    //let format = match args.flag_format.as_ref() {
    //    "latex" => Format::Latex,
    //    "html"  => Format::Html,
    //    _       => panic!("Unknow ouput format: {}", args.flag_format)
    //};

    //let config = Config{
    //    format: format,
    //    with_header_and_footer: args.flag_ready,
    //};

    //match process(file, output, config) {
    //    Ok(_)   => {},
    //    Err(e)  => panic!("{}", e),
    //}
}
