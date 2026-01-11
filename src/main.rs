use clap::{Parser, ValueEnum};
use inquire::Select;
use std::{fmt, fs::OpenOptions};

#[derive(Debug, Clone, Parser, ValueEnum, PartialEq)]
enum Cmd {
    ScanLive,
    ScanFile,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long = "command", help = "command for scanning ")]
    pub command: Option<Cmd>,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match self {
            Cmd::ScanFile => "scan a file",
            Cmd::ScanLive => "Live scan packets",
        };
        write!(f, "{}", label)
    }
}

fn main() {
    let args = Args::parse();
    let command = args.command.unwrap_or_else(|| {
        Select::new("Choose command:", vec![Cmd::ScanLive, Cmd::ScanFile])
            .prompt()
            .expect("Prompt failed")
    });
    println!("{:?}", command);
    if command=="ScanLive"{

    } else{
        
    }
}
