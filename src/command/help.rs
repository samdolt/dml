use docopt::Docopt;
use ::Command;
use command::DmlCommand;

pub struct HelpCommand;

const USAGE: &'static str = "
Usage:
    dml help <command>

Example:
    dml help build
";

#[derive(RustcDecodable)]
struct Args {
    arg_command: Command,
}

impl DmlCommand for HelpCommand {
    fn help() -> String {
        USAGE.to_string()
    }

    fn run(argv: &Vec<String>){

        let args: Args = Docopt::new(USAGE)
                                .and_then(|d| d.argv(argv.into_iter()).decode())
                                .unwrap_or_else(|e| e.exit());

        println!("{}", args.arg_command.help());
    }
}
