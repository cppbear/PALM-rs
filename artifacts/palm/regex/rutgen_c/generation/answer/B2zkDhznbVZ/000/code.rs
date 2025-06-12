// Answer 0

#[test]
fn test_parser_new() {
    let parser = Parser::new();
    assert_eq!(parser.nest_limit, 250);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, false);
    assert_eq!(parser.capture_index.get(), 0);
    assert_eq!(parser.pos.get().offset, 0);
    assert_eq!(parser.pos.get().line, 1);
    assert_eq!(parser.pos.get().column, 1);
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
    assert!(parser.capture_names.borrow().is_empty());
    assert_eq!(parser.scratch.borrow().as_str(), "");
}

#[test]
fn test_parser_builder_new() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.nest_limit, 250);
    assert_eq!(builder.octal, false);
    assert_eq!(builder.ignore_whitespace, false);
}

#[test]
fn test_parser_builder_build() {
    let builder = ParserBuilder::new();
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 250);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, false);
    assert_eq!(parser.capture_index.get(), 0);
    assert_eq!(parser.pos.get().offset, 0);
    assert_eq!(parser.pos.get().line, 1);
    assert_eq!(parser.pos.get().column, 1);
}

