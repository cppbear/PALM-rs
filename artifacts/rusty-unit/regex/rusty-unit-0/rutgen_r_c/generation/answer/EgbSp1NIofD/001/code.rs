// Answer 0

#[test]
fn test_parser_builder_default() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 10,
        octal: true,
    };
    let parser = builder.build();
    assert_eq!(parser.pos.get(), Position { offset: 0, line: 1, column: 1 });
    assert_eq!(parser.capture_index.get(), 0);
    assert_eq!(parser.nest_limit, 10);
    assert!(parser.octal);
    assert!(!parser.initial_ignore_whitespace);
    assert!(!parser.ignore_whitespace.get());
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
    assert!(parser.capture_names.borrow().is_empty());
    assert_eq!(parser.scratch.borrow().as_str(), "");
}

#[test]
fn test_parser_builder_with_ignore_whitespace() {
    let builder = ParserBuilder {
        ignore_whitespace: true,
        nest_limit: 5,
        octal: false,
    };
    let parser = builder.build();
    assert!(parser.initial_ignore_whitespace);
    assert!(!parser.octal);
}

#[test]
fn test_parser_builder_min_nest_limit() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 0,
        octal: true,
    };
    let parser = builder.build();
    assert_eq!(parser.nest_limit, 0);
    assert!(parser.octal);
}

#[test]
fn test_parser_builder_max_nest_limit() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: u32::MAX,
        octal: false,
    };
    let parser = builder.build();
    assert_eq!(parser.nest_limit, u32::MAX);
    assert!(!parser.octal);
}

