use docopt::Docopt;
use command::DmlCommand;
use helpers::BuildDir;

pub struct CleanCommand;

const USAGE: &'static str = "
Usage:
    dml clean

Remove build directory
";

#[derive(Debug, RustcDecodable)]
struct Args;

impl DmlCommand for CleanCommand {
    fn help() -> String {
        USAGE.to_string()
    }

    fn run(argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                          .and_then(|d| d.argv(argv.into_iter()).decode())
                          .unwrap_or_else(|e| e.exit());

        let dir = BuildDir::new().expect("Error - Build dir not deleted");
        dir.delete().expect("Error - Build dir not deleted");
    }
}
