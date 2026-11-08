use std::fmt;

// O #[derive(...)] diz ao compilador para criar automaticamente duas funcionalidades:
// 1. PartialEq -> Permite usar == e != comparando todos os campos da struct.
// 2. Debug     -> Permite imprimir a struct inteira com {:?} para testes e logs.
#[derive(PartialEq, Debug)]
pub enum Command {
    Init,
    Migrate,
    Info,
    Validate,
    Repair,
}

#[derive(PartialEq, Debug)]
pub enum CliError {
    ComandoNaoReconhecido,
}

// implementa o toString no enum CliError, ensina ao rust como exibir as descrições do erro
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComandoNaoReconhecido => write!(f, "Comando não reconhecido."),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::cli::{CliError::ComandoNaoReconhecido, Command, get_command};

    #[test]
    fn deve_retornar_command_info() {
        assert_eq!(get_command("info").unwrap(), Command::Info)
    }

    #[test]
    fn deve_retornar_command_init() {
        assert_eq!(get_command("init").unwrap(), Command::Init)
    }

    #[test]
    fn deve_retornar_command_migrate() {
        assert_eq!(get_command("migrate").unwrap(), Command::Migrate)
    }

    #[test]
    fn deve_retornar_command_repair() {
        assert_eq!(get_command("repair").unwrap(), Command::Repair)
    }

    #[test]
    fn deve_retornar_command_validate() {
        assert_eq!(get_command("validate").unwrap(), Command::Validate)
    }

    #[test]
    fn deve_retornar_command_error() {
        assert_eq!(get_command("invalido"), Err(ComandoNaoReconhecido))
    }
}
