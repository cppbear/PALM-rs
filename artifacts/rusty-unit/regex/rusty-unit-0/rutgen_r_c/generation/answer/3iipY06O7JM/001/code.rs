// Answer 0

#[test]
fn test_automatic_with_default_state() {
    let builder = ExecBuilder::new("test");
    let modified_builder = builder.automatic();
    assert!(modified_builder.match_type.is_none());
    assert_eq!(modified_builder.options.pats.len(), 1);
    assert_eq!(modified_builder.options.pats[0], "test");
}

#[test]
fn test_automatic_after_nfa() {
    let builder = ExecBuilder::new("test").nfa();
    let modified_builder = builder.automatic();
    assert!(modified_builder.match_type.is_none());
    assert_eq!(modified_builder.options.pats.len(), 1);
    assert_eq!(modified_builder.options.pats[0], "test");
}

#[test]
fn test_automatic_after_bounded_backtracking() {
    let builder = ExecBuilder::new("test").bounded_backtracking();
    let modified_builder = builder.automatic();
    assert!(modified_builder.match_type.is_none());
    assert_eq!(modified_builder.options.pats.len(), 1);
    assert_eq!(modified_builder.options.pats[0], "test");
} 

#[test]
fn test_automatic_with_multiple_patterns() {
    let builder = ExecBuilder::new_many(vec!["test", "sample", "example"]);
    let modified_builder = builder.automatic();
    assert!(modified_builder.match_type.is_none());
    assert_eq!(modified_builder.options.pats.len(), 3);
    assert_eq!(modified_builder.options.pats[0], "test");
    assert_eq!(modified_builder.options.pats[1], "sample");
    assert_eq!(modified_builder.options.pats[2], "example");
}

