use rustway::engine::init::init_at;
use tempfile::tempdir;

#[test]
fn deve_criar_pasta_migrations_e_config() {
    let tmp_dir = tempdir().unwrap();
    init_at(tmp_dir.path()).unwrap();

    assert!(tmp_dir.path().join("migrations").is_dir());
    assert!(tmp_dir.path().join("rustway.toml").is_file());
}
