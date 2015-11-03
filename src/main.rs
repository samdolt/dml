extern crate dml;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::io::stdout;
use std::io::Write;
use std::fs::File;

use dml::process;

const USAGE: &'static str = "
DML - Dolt Markup Language

Usage:
  dml <input>
  dml <input> --output=<output>
  dml --version
  dml --help

Options:
  -h --help                 Show this screen.
  -v --version              Show version
  -o --output=<output>      Specify output file [default: -]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input: String,
    flag_output: String,
    flag_version: bool,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let file = match File::open(&args.arg_input) {
        Ok(file) => file,
        Err(..)  => panic!("{} doesn't exist", args.arg_input)
    };

    let output: Box<Write> = if &args.flag_output == "-" {
            Box::new(stdout())
        } else {
            match File::create(&args.flag_output) {
                Ok(file)    => Box::new(file),
                Err(..)     => panic!("Couldn't create file {}", args.flag_output),
        }
    };

    match process(file, output) {
        Ok(_)   => {},
        Err(e)  => panic!("{}", e),
    }
}
