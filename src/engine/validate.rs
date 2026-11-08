use crate::error::RustwayError;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

static RE_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^V[0-9]+$").expect("Expressão regular de versão inválida"));

pub enum ValidationSeverity {
    Error,
    Warning,
}

pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub file: String,
    pub message: String,
}

pub fn validate() -> Result<(), RustwayError> {
    let cur_dir = env::current_dir()?;
    validate_at(&cur_dir)
}

pub fn validate_at(base_dir: &std::path::Path) -> Result<(), RustwayError> {
    let migrations = read_migrations_folder(base_dir)?;

    println!("{:<10} {:<30} {}", "ERRO", "DESCRIÇÃO", "ARQUIVO");

    for migration in migrations {
        for issue in validate_migration_filename(migration.as_str()) {
            match issue.severity {
                ValidationSeverity::Error => {
                    println!("{:<10} {:<30} {}", "error", issue.message, issue.file)
                }
                ValidationSeverity::Warning => {
                    println!("{:<10} {:<30} {}", "warning", issue.message, issue.file)
                }
            }
        }
    }

    Ok(())
}

pub fn read_migrations_folder(base_dir: &Path) -> Result<Vec<String>, RustwayError> {
    let path = base_dir.join("migrations");

    if !path.is_dir() {
        return Err(RustwayError::MigrationsDirectoryNotFound);
    }

    let mut migrations = Vec::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let file_path = entry.path();

        if !file_path.is_file() || !file_path.extension().is_some_and(|ext| ext == "sql") {
            continue;
        }

        let Some(filename) = file_path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        migrations.push(filename.to_string());
    }

    Ok(migrations)
}

fn validate_migration_filename(filename: &str) -> Vec<ValidationIssue> {
    let mut validations = Vec::new();

    let Some((version, description)) = filename.split_once("__") else {
        validations.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            file: filename.to_string(),
            message: "versão e descrição das migrarions devem ser separadas po duplo _".to_string(),
        });

        return validations;
    };

    if !RE_VERSION.is_match(version) {
        validations.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            file: filename.to_string(),
            message: "versionamento inválido padrão esperado V[0-9]+__".to_string(),
        });
    }

    let Some(without_extension) = description.strip_suffix(".sql") else {
        validations.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            file: filename.to_string(),
            message: "arquivo sem .sql".to_string(),
        });

        return validations;
    };

    if without_extension.is_empty() {
        validations.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            file: filename.to_string(),
            message: "o arquivo de migration deve conter uma descrição clara".to_string(),
        });
    }

    if without_extension.chars().any(char::is_whitespace) {
        validations.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            file: filename.to_string(),
            message: "a descrição da migração não deve conter espaços em branco utilize _"
                .to_string(),
        });
    }

    validations
}
