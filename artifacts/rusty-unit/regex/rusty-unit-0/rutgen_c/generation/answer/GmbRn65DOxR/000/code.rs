// Answer 0

#[test]
fn test_new_many_single_expression() {
    let result = ExecBuilder::new_many(vec!["abc"]);
    assert_eq!(result.options.pats.len(), 1);
    assert_eq!(result.options.pats[0], "abc");
}

#[test]
fn test_new_many_multiple_expressions() {
    let result = ExecBuilder::new_many(vec!["abc", "123", ".*"]);
    assert_eq!(result.options.pats.len(), 3);
    assert_eq!(result.options.pats[0], "abc");
    assert_eq!(result.options.pats[1], "123");
    assert_eq!(result.options.pats[2], ".*");
}

#[test]
fn test_new_many_empty_expressions() {
    let result = ExecBuilder::new_many(vec![]);
    assert_eq!(result.options.pats.len(), 0);
}

#[test]
fn test_new_many_whitespace_expression() {
    let result = ExecBuilder::new_many(vec!["   ", "\n\t"]);
    assert_eq!(result.options.pats.len(), 2);
    assert_eq!(result.options.pats[0], "   ");
    assert_eq!(result.options.pats[1], "\n\t");
}

#[test]
fn test_new_many_special_characters() {
    let result = ExecBuilder::new_many(vec![r"\d+", r"[a-zA-Z]", ".*?"]);
    assert_eq!(result.options.pats.len(), 3);
    assert_eq!(result.options.pats[0], r"\d+");
    assert_eq!(result.options.pats[1], r"[a-zA-Z]");
    assert_eq!(result.options.pats[2], ".*?");
}

