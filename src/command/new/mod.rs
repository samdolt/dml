use docopt::Docopt;
use subcmd::Command;

pub struct NewCommand;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::Write;

const USAGE: &'static str = "
Create a new project at path
Usage:
    dml new <path>
";

const SUMMARY: &'static str = include_str!("summary.toml");

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: String,
}

impl Command for NewCommand {
    fn run(&self, argv: &Vec<String>) {

        let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.argv(argv.into_iter()).decode())
                             .unwrap_or_else(|e| e.exit());

        // Create project directory
        fs::create_dir_all(&args.arg_path).expect("Couln'd create project directory");

        // Put template file
        let mut sum_path = PathBuf::from(&args.arg_path);
        sum_path.push("summary.toml");

        let mut sum = File::create(sum_path).expect("Couldn't create summary.toml");
        sum.write_all(SUMMARY.as_bytes()).expect("Couln't write summary.toml");
    }

    fn name<'a>(&self) -> &'a str {
        "new"
    }
    fn help<'a>(&self) -> &'a str {
        USAGE
    }
    fn description<'a>(&self) -> &'a str {
        "Create a new dml project"
    }
}
