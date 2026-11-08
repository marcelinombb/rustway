use std::fmt;

#[derive(Debug)]
pub enum RustwayError {
    ConfigAlreadyExists,
    MigrationsDirectoryNotFound,
    InvalidMigrationName,
    DuplicateMigrationVersion,
    NotAFolder,
    Io(std::io::Error),
}

impl fmt::Display for RustwayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigAlreadyExists => write!(
                f,
                "Erro: o arquivo rustway.toml já existe.\nNenhum arquivo de configuração foi sobrescrito."
            ),
            Self::MigrationsDirectoryNotFound => {
                write!(f, "Erro: pasta de migrações não encontrada.")
            }
            Self::NotAFolder => write!(f, "Erro: não é uma pasta!"),
            Self::InvalidMigrationName => write!(f, "Erro: migração com nome invalido."),
            Self::DuplicateMigrationVersion => {
                write!(f, "Erro: duas migrations possuem a mesma versão..")
            }
            Self::Io(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for RustwayError {
    fn from(error: std::io::Error) -> Self {
        RustwayError::Io(error)
    }
}
