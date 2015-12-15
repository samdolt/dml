use docopt::Docopt;

use std::io::stdout;
use std::io::Write;
use std::fs::File;

use subcmd::Command;


pub struct BuildCommand;

const USAGE: &'static str = "
Usage:
    dml build

Example:
    dml build
";

#[derive(RustcDecodable)]
struct Args;

impl Command for BuildCommand {
    

    fn run(&self, argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                                .and_then(|d| d.argv(argv.into_iter()).decode())
                                .unwrap_or_else(|e| e.exit());

   }

    fn name<'a>(&self)  -> &'a str { "build" }
    fn description<'a>(&self) -> &'a str { "Compile the current project" }
    fn help<'a>(&self) -> &'a str { USAGE }
}
