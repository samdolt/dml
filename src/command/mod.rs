#[derive(Debug, RustcDecodable)]
pub enum Command {
    Build, New, Clean, Process,
}

trait DmlCommand {
    fn run(args: &Vec<String>);
}

mod process;
use self::process::ProcessCommand;

pub fn run(cmd: Command, args: &Vec<String>) {
    match cmd {
        Command::Build      =>  println!("Build"),
        Command::New        =>  println!("New"),
        Command::Clean      =>  println!("Clean"),
        Command::Process    =>  ProcessCommand::run(&args),
    }
}
