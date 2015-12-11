use docopt::Docopt;
use ::Command;
use command::DmlCommand;

use std::io::stdout;
use std::io::Write;
use std::fs::File;


pub struct BuildCommand;

const USAGE: &'static str = "
Usage:
    dml build

Example:
    dml build
";

#[derive(RustcDecodable)]
struct Args;

impl DmlCommand for BuildCommand {
    fn help() -> String {
        USAGE.to_string()
    }

    fn run(argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                                .and_then(|d| d.argv(argv.into_iter()).decode())
                                .unwrap_or_else(|e| e.exit());

   }
}
