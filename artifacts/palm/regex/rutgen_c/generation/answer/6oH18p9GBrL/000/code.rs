// Answer 0

#[test]
fn test_exec_builder_only_utf8_default() {
    let exec_builder = ExecBuilder::new("test");
    assert_eq!(exec_builder.only_utf8, true);
}

#[test]
fn test_exec_builder_only_utf8_set_true() {
    let exec_builder = ExecBuilder::new("test").only_utf8(true);
    assert_eq!(exec_builder.only_utf8, true);
}

#[test]
fn test_exec_builder_only_utf8_set_false() {
    let exec_builder = ExecBuilder::new("test").only_utf8(false);
    assert_eq!(exec_builder.only_utf8, false);
}

#[test]
fn test_exec_builder_only_utf8_chain() {
    let exec_builder = ExecBuilder::new("test")
        .only_utf8(false)
        .only_utf8(true);
    assert_eq!(exec_builder.only_utf8, true);
}

