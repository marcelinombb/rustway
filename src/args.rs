use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rustway",
    version,
    about = "Ferramenta de migrations para bancos de dados"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Migrate,
    Info,
    Validate,
    Repair,
}
