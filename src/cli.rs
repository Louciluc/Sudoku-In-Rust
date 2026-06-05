mod editterm;
use editterm::*;
mod printgrid;
use printgrid::*;
mod printsol;
use printsol::*;
mod readtxt;
use readtxt::*;
mod savetxt;
use savetxt::*;
mod solve;
use solve::*;

enum Command {
    EditTerm(EditTermCmd),
    PrintGrid(PrintGridCmd),
    PrintSolutions(PrintSolCmd),
    ReadTxt(ReadTxtCmd),
    SaveTxt(SaveTxtCmd),
    Solve(SolveCmd),
}

pub use crate::*;
pub fn cli_main(args: &Vec<String>) {
    use crate::cli::Command;
    if args[0] == "--help" || args[0] == "-h" {
        print!("{}", help_string());
        return;
    }
    let mut cmds: Vec<Command> = Vec::new();
    for arg in args {
        //println!("{}", arg);
        if arg.starts_with("-") { continue; }
        match arg.as_str() {
            EditTermCmd::NAME => {
                cmds.push(Command::EditTerm(EditTermCmd::parse(args)));
            },
            PrintGridCmd::NAME => {
                cmds.push(Command::PrintGrid(PrintGridCmd::parse(args)));
            },
            PrintGridCmd::NAME => {
                cmds.push(Command::PrintSolutions(PrintSolCmd::parse(args)));
            },
            ReadTxtCmd::NAME => {
                cmds.push(Command::ReadTxt(ReadTxtCmd::parse(args)));
            },
            SaveTxtCmd::NAME => {
                cmds.push(Command::SaveTxt(SaveTxtCmd::parse(args)));
            },
            SolveCmd::NAME => {
                cmds.push(Command::Solve(SolveCmd::parse(args)));
            },
            _ => { panic!("command couldnt be parsed. Please check for mistypes."); }
        }
    }
}

pub trait Cmd {
    fn parse(args: &Vec<String>) -> Self;
    fn execute(&self, g: &mut Grid);
    fn help_string() -> String;
}

pub fn help_string() -> String {
    return String::from("TODO: Helper function");
}
