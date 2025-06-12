// Answer 0

#[test]
fn test_parser_new_default_configuration() {
    // Create a new parser with default configuration.
    let parser = Parser::new();

    // Check that the position is initialized correctly.
    assert_eq!(parser.pos.get().offset, 0);
    assert_eq!(parser.pos.get().line, 1);
    assert_eq!(parser.pos.get().column, 1);

    // Check that the capture index is initialized to 0.
    assert_eq!(parser.capture_index.get(), 0);

    // Check that the nest limit is set to the default value (250).
    assert_eq!(parser.nest_limit, 250);

    // Check that octal is set to false by default.
    assert!(!parser.octal);

    // Check that the initial ignore whitespace is set to false.
    assert!(!parser.initial_ignore_whitespace);

    // Check that ignore whitespace is set to false by default.
    assert!(!parser.ignore_whitespace.get());

    // Check that comments list is initialized empty.
    assert!(parser.comments.borrow().is_empty());

    // Check that the stack of grouped sub-expressions is initialized empty.
    assert!(parser.stack_group.borrow().is_empty());

    // Check that the stack of nested character classes is initialized empty.
    assert!(parser.stack_class.borrow().is_empty());

    // Check that capture names list is initialized empty.
    assert!(parser.capture_names.borrow().is_empty());

    // Check that the scratch buffer is initialized to an empty string.
    assert!(parser.scratch.borrow().is_empty());
}

