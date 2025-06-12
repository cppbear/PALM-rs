// Answer 0

#[derive(Debug)]
struct TestStruct {
    config: String,
}

impl TestStruct {
    fn new(config: String) -> Self {
        TestStruct { config }
    }

    fn config(&self) -> &String {
        &self.config
    }
}

#[test]
fn test_config_returns_correct_reference() {
    let test_instance = TestStruct::new("test_config".to_string());
    let config_reference = test_instance.config();
    assert_eq!(config_reference, "test_config");
}

#[test]
fn test_config_with_empty_string() {
    let test_instance = TestStruct::new("".to_string());
    let config_reference = test_instance.config();
    assert_eq!(config_reference, "");
}

#[test]
fn test_config_with_large_string() {
    let large_config = "a".repeat(1_000_000); // 1 million characters
    let test_instance = TestStruct::new(large_config.clone());
    let config_reference = test_instance.config();
    assert_eq!(config_reference, &large_config);
}

