// Answer 0

#[test]
fn test_pop_group_valid_group() {
    let mut group_concat = ast::Concat {
        span: ast::Span {
            start: ast::Position { offset: 0, line: 1, column: 1 },
            end: ast::Position { offset: 1, line: 1, column: 2 },
        },
        asts: Vec::new(),
    };

    let parser = Parser {
        pos: Cell::new(ast::Position { offset: 1, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(vec![
            ast::GroupState::Group {
                concat: group_concat.clone(),
                group: ast::Group {
                    span: ast::Span {
                        start: ast::Position { offset: 0, line: 1, column: 1 },
                        end: ast::Position { offset: 1, line: 1, column: 2 },
                    },
                    kind: ast::GroupKind::Normal,
                    ast: Box::new(ast::Ast::Empty(ast::Span {
                        start: ast::Position { offset: 0, line: 1, column: 1 },
                        end: ast::Position { offset: 1, line: 1, column: 2 },
                    })),
                },
                ignore_whitespace: false,
            },
        ]),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI { parser: &parser, pattern: ")" };

    let result = parser_instance.pop_group(group_concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "unopened group")]
fn test_pop_group_no_open_group() {
    let group_concat = ast::Concat {
        span: ast::Span {
            start: ast::Position { offset: 0, line: 1, column: 1 },
            end: ast::Position { offset: 1, line: 1, column: 2 },
        },
        asts: Vec::new(),
    };

    let parser = Parser {
        pos: Cell::new(ast::Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI { parser: &parser, pattern: ")" };

    let _ = parser_instance.pop_group(group_concat);
}

#[test]
fn test_pop_group_with_alternation() {
    let mut group_concat = ast::Concat {
        span: ast::Span {
            start: ast::Position { offset: 0, line: 1, column: 1 },
            end: ast::Position { offset: 1, line: 1, column: 2 },
        },
        asts: Vec::new(),
    };

    let alt = ast::Alternation {
        span: ast::Span {
            start: ast::Position { offset: 0, line: 1, column: 1 },
            end: ast::Position { offset: 1, line: 1, column: 2 },
        },
        asts: Vec::new(),
    };

    let parser = Parser {
        pos: Cell::new(ast::Position { offset: 1, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(vec![
            ast::GroupState::Alternation(alt),
            ast::GroupState::Group {
                concat: group_concat.clone(),
                group: ast::Group {
                    span: ast::Span {
                        start: ast::Position { offset: 0, line: 1, column: 1 },
                        end: ast::Position { offset: 1, line: 1, column: 2 },
                    },
                    kind: ast::GroupKind::Normal,
                    ast: Box::new(ast::Ast::Empty(ast::Span {
                        start: ast::Position { offset: 0, line: 1, column: 1 },
                        end: ast::Position { offset: 1, line: 1, column: 2 },
                    })),
                },
                ignore_whitespace: false,
            },
        ]),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI { parser: &parser, pattern: ")" };

    let result = parser_instance.pop_group(group_concat);
    assert!(result.is_ok());
}

