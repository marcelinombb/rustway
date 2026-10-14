use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;

pub fn init() -> std::io::Result<()> {
    let cur_dir = env::current_dir()?;
    init_at(&cur_dir)?;
    Ok(())
}

pub fn init_at(base_dir: &std::path::Path) -> std::io::Result<()> {
    create_migrations_dir(&base_dir)?;
    create_config_file(&base_dir)?;
    Ok(())
}

fn create_migrations_dir(base_dir: &std::path::Path) -> std::io::Result<()> {
    let mut path = base_dir.to_path_buf();
    path.push("migrations");

    match fs::create_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == AlreadyExists && path.is_dir() => Ok(()),
        Err(e) => Err(e),
    }
}

fn create_config_file(base_dir: &std::path::Path) -> std::io::Result<()> {
    let mut path = base_dir.to_path_buf();
    path.push("rustway.toml");

    let file = OpenOptions::new().write(true).create_new(true).open(path);
    let mut file = file?;
    file.write_all(b"[migrations]\ndirectory = \"migrations\"\n")?;
    Ok(())
}
