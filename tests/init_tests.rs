use rustway::engine::init::init_at;
use std::fs;
use std::fs::create_dir;
use tempfile::tempdir;

#[test]
fn deve_criar_pasta_migrations_e_config() {
    let tmp_dir = tempdir().unwrap();
    init_at(tmp_dir.path()).unwrap();

    assert!(tmp_dir.path().join("migrations").is_dir());
    assert!(tmp_dir.path().join("rustway.toml").is_file());
}

#[test]
fn deve_aceitar_pasta_migrations_existente() {
    let tmp_dir = tempdir().unwrap();
    create_dir(tmp_dir.path().join("migrations")).unwrap();
    assert!(init_at(tmp_dir.path()).is_ok())
}

#[test]
fn nao_deve_sobrescrever_config_existente() {
    let tmp_dir = tempdir().unwrap();

    fs::write(
        tmp_dir.path().join("rustway.toml"),
        "configuracao existente",
    )
    .unwrap();

    assert!(init_at(tmp_dir.path()).is_err());
    assert_eq!(
        std::fs::read_to_string(tmp_dir.path().join("rustway.toml")).unwrap(),
        "configuracao existente"
    );
}
