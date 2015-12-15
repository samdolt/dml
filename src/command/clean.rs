use docopt::Docopt;
use helpers::BuildDir;

use subcmd::Command;

pub struct CleanCommand;

const USAGE: &'static str = "
Usage:
    dml clean

Remove build directory
";

#[derive(Debug, RustcDecodable)]
struct Args;

impl Command for CleanCommand {
    fn run(&self, argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                          .and_then(|d| d.argv(argv.into_iter()).decode())
                          .unwrap_or_else(|e| e.exit());

        let dir = BuildDir::new().expect("Error - Build dir not deleted");
        dir.delete().expect("Error - Build dir not deleted");
    }

    fn name<'a>(&self) -> &'a str { "clean" }
    fn help<'a>(&self) -> &'a str { USAGE }
    fn description<'a>(&self) -> &'a str { "Remove the target directory" }
}
