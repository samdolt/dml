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

use dml::process;
use dml::Format;
use dml::Config;

const USAGE: &'static str = "
DML - Dolt Markup Language

Usage:
  dml [--format=<format>]
  dml <input> [--format=<format>] [-r|--ready]
  dml <input> --output=<output> [--format=<format>] [-r|--ready]
  dml --version
  dml --help

Options:
  -h --help                 Show this screen.
  -v --version              Show version
  -o --output=<output>      Specify output file [default: -]
  -f --format=<format>      Specify output format [default: html]
  -r --ready                With header and footer
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input: String,
    flag_output: String,
    flag_format: String,
    flag_version: bool,
    flag_ready: bool,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    env_logger::init().unwrap();

    if args.arg_input == "" {
        // We are in project mode
        project_mode(&args);
    } else {
        // We are in single file mode
        single_file_mode(&args);
    }
}

fn project_mode(args: &Args) {
    unimplemented!()
}

fn single_file_mode(args: &Args) {
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


    let format = match args.flag_format.as_ref() {
        "latex" => Format::Latex,
        "html"  => Format::Html,
        _       => panic!("Unknow ouput format: {}", args.flag_format)
    };

    let config = Config{
        format: format,
        with_header_and_footer: args.flag_ready,
    };

    match process(file, output, config) {
        Ok(_)   => {},
        Err(e)  => panic!("{}", e),
    }
}
