// Answer 0

#[test]
fn test_exec_builder_bytes_setting_true() {
    let exec_builder = ExecBuilder::new("test");
    let updated_exec_builder = exec_builder.bytes(true);
    assert!(updated_exec_builder.bytes);
}

#[test]
fn test_exec_builder_bytes_setting_false() {
    let exec_builder = ExecBuilder::new("test");
    let updated_exec_builder = exec_builder.bytes(false);
    assert!(!updated_exec_builder.bytes);
}

#[test]
fn test_exec_builder_bytes_setting_multiple() {
    let exec_builder = ExecBuilder::new("test");
    
    let updated_exec_builder_true = exec_builder.bytes(true);
    assert!(updated_exec_builder_true.bytes);
    
    let updated_exec_builder_false = updated_exec_builder_true.bytes(false);
    assert!(!updated_exec_builder_false.bytes);
}

