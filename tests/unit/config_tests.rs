use hta_gen::config::load_config;

#[test]
fn test_load_config() {
    let config_content = r#"
        [settings]
        log_level = "debug"
    "#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), config_content).unwrap();

    let config = load_config(temp_file.path().to_str().unwrap()).unwrap();
    let log_level = config.get("settings").and_then(|s| s.get("log_level")).unwrap();

    assert_eq!(log_level.as_str().unwrap(), "debug");
}
