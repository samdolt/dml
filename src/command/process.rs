use docopt::Docopt;
use command::DmlCommand;

pub struct ProcessCommand;

use std::fs::File;
use std::io::Write;
use std::io::stdout;

use Format;
use Config;
use process;

const USAGE: &'static str = "
Usage:
    dml process <input> [<output>] [--format=<fmt> | --partial]
    dml process --help

Options:
    -h --help          Show this screen.
    -f --format=<fmt>  Output format. [default: html]
    -p --partial       Enable partial output

Example:
    dml process input_file.dml output_file.html
    dml process in.dml out.tex --format=latex

Notes:
    Partial output let you get html or latex without header
    and without footer. It is useful if you want to insert
    this file in another file.
";

#[derive(RustcDecodable)]
struct Args {
    arg_input: String,
    arg_output: Option<String>,
    flag_format: Format,
    flag_partial: bool,
}

impl DmlCommand for ProcessCommand {
    fn help() -> String {
        USAGE.to_string()
    }

    fn run(argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                                .and_then(|d| d.argv(argv.into_iter()).decode())
                                .unwrap_or_else(|e| e.exit());

        let input = match File::open(args.arg_input) {
            Ok(file)    => file,
            Err(..)     => panic!("Couln't read file"),
        };

        let output: Box<Write> = match args.arg_output {
            Some(path) =>   Box::new(
                                File::create(path)
                                .expect("Couldn't load output file")
                            ),
            None       =>   Box::new(stdout()),
        };

        let format = Format::Html;
        let config = Config {
            format: args.flag_format,
            with_header_and_footer: !args.flag_partial,
        };

        match process(input, output, config) {
            Ok(_)   =>  {},
            Err(e)  => panic!("{}", e),
        }
    }
}
