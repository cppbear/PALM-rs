// Answer 0

#[test]
fn test_only_utf8_enabled() {
    let re = "test";
    let exec_builder = ExecBuilder::new(re);
    let updated_builder = exec_builder.only_utf8(true);
}

#[test]
fn test_only_utf8_disabled() {
    let re = "test";
    let exec_builder = ExecBuilder::new(re);
    let updated_builder = exec_builder.only_utf8(false);
}

#[test]
fn test_only_utf8_with_multiple_patterns() {
    let patterns = vec!["pattern1", "pattern2"];
    let exec_builder = ExecBuilder::new_many(patterns);
    let updated_builder = exec_builder.only_utf8(true);
}

#[test]
fn test_only_utf8_with_empty_pattern() {
    let patterns = vec![""];
    let exec_builder = ExecBuilder::new_many(patterns);
    let updated_builder = exec_builder.only_utf8(false);
}

