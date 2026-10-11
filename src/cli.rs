pub enum Command {
    Init,
    Migrate,
    Info,
    Validate,
    Repair,
}

pub enum CliError {
    ComandoNaoReconhecido,
}

fn command_show(command: Command) {
    match command {
        Command::Init => println!("Init"),
        Command::Info => println!("Info"),
        Command::Migrate => println!("Migrate"),
        Command::Validate => println!("Validate"),
        Command::Repair => println!("Repair"),
    }
}

pub fn get_command(arg: &str) -> Result<Command, CliError> {
    match arg {
        "init" => Ok(Command::Init),
        "info" => Ok(Command::Info),
        "migrate" => Ok(Command::Migrate),
        "validate" => Ok(Command::Validate),
        "repair" => Ok(Command::Repair),
        _ => Err(CliError::ComandoNaoReconhecido),
    }
}

pub fn run(command: Command) {
    command_show(command);
}
