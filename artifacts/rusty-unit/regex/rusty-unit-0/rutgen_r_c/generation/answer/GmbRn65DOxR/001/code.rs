// Answer 0

#[test]
fn test_new_many_with_single_pattern() {
    let result = ExecBuilder::new_many(vec!["a"]);
    assert_eq!(result.options.pats.len(), 1);
    assert_eq!(result.options.pats[0], "a");
}

#[test]
fn test_new_many_with_multiple_patterns() {
    let result = ExecBuilder::new_many(vec!["a", "b", "c"]);
    assert_eq!(result.options.pats.len(), 3);
    assert_eq!(result.options.pats[0], "a");
    assert_eq!(result.options.pats[1], "b");
    assert_eq!(result.options.pats[2], "c");
}

#[test]
fn test_new_many_with_empty_patterns() {
    let result = ExecBuilder::new_many(vec![]);
    assert_eq!(result.options.pats.len(), 0);
}

#[test]
fn test_new_many_with_whitespace_pattern() {
    let result = ExecBuilder::new_many(vec!["   "]);
    assert_eq!(result.options.pats.len(), 1);
    assert_eq!(result.options.pats[0], "   ");
}

#[test]
fn test_new_many_with_special_characters() {
    let result = ExecBuilder::new_many(vec![".*", "\\d+", "\\w"]);
    assert_eq!(result.options.pats.len(), 3);
    assert_eq!(result.options.pats[0], ".*");
    assert_eq!(result.options.pats[1], "\\d+");
    assert_eq!(result.options.pats[2], "\\w");
}

#[test]
fn test_new_many_with_unicode_pattern() {
    let result = ExecBuilder::new_many(vec!["你好", "こんにちは"]);
    assert_eq!(result.options.pats.len(), 2);
    assert_eq!(result.options.pats[0], "你好");
    assert_eq!(result.options.pats[1], "こんにちは");
}

