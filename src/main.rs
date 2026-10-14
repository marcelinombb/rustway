mod args;
use args::{Cli, Commands};
use clap::Parser;
use std::process::ExitCode;
mod engine;
use engine::init::init;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info => println!("Info"),
        Commands::Init => init().unwrap(),
        Commands::Migrate => print!("Migrate"),
        Commands::Repair => print!("Repair"),
        Commands::Validate => print!("Validate"),
    }

    ExitCode::SUCCESS
}
