use clap::Parser;
use rustway::args::{Cli, Commands};
use rustway::engine::info::info;
use rustway::engine::init::init;
use rustway::engine::validate::validate;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => match info() {
            Ok(()) => {
                println!("Rustway info.");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        Commands::Init => match init() {
            Ok(()) => {
                println!("Rustway inicializado com sucesso.");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        Commands::Migrate => {
            println!("Migrate");
            ExitCode::SUCCESS
        }
        Commands::Repair => {
            println!("Repair");
            ExitCode::SUCCESS
        }
        Commands::Validate => match validate() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
    }
}
