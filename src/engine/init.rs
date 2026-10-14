use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;

pub fn init() -> std::io::Result<()> {
    create_migrations_dir()?;
    create_config_file()?;
    Ok(())
}

fn create_migrations_dir() -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("migrations");

    match fs::create_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == AlreadyExists && path.is_dir() => Ok(()),
        Err(e) => Err(e),
    }
}

fn create_config_file() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("rustway.toml");

    match file {
        Ok(mut file) => {
            file.write_all(b"[migrations]\ndirectory = \"migrations\"\n")?;
            return Ok(());
        }
        Err(e) => Err(e),
    }
}
