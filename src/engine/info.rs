use crate::error::RustwayError;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Migration {
    pub path: PathBuf,
    pub version: u32,
    pub description: String,
}

pub fn info() -> Result<(), RustwayError> {
    let cur_dir = env::current_dir()?;
    info_at(&cur_dir)
}

pub fn info_at(base_dir: &std::path::Path) -> Result<(), RustwayError> {
    let migrations = read_migrations_folder(&base_dir)?;

    println!("{:<10} {:<30} {}", "VERSÃO", "DESCRIÇÃO", "ARQUIVO");

    for migration in migrations {
        println!(
            "{:<10} {:<30} {}",
            migration.version,
            migration.description,
            migration
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
    }

    Ok(())
}

pub fn read_migrations_folder(base_dir: &Path) -> Result<Vec<Migration>, RustwayError> {
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

        let Some((version, description)) = parse_migration_name(filename) else {
            continue;
        };

        migrations.push(Migration {
            path: file_path,
            version,
            description,
        });
    }

    migrations.sort_by_key(|migration| migration.version);

    Ok(migrations)
}

fn parse_migration_name(filename: &str) -> Option<(u32, String)> {
    let without_prefix = filename.strip_prefix('V')?;
    let (version, description) = without_prefix.split_once("__")?;

    let version = version.parse::<u32>().ok()?;

    let description = description.strip_suffix(".sql")?;

    if description.is_empty() {
        return None;
    }

    Some((version, description.to_string()))
}
