// Answer 0

#[test]
fn test_parse_primitive_dot() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: ".",
    };

    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(ast) = result {
        if let Primitive::Dot(_) = ast {
            assert!(true);
        } else {
            assert!(false, "Expected Primitive::Dot");
        }
    }
}

#[test]
fn test_parse_primitive_start_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "^",
    };

    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(ast) = result {
        if let Primitive::Assertion(assertion) = ast {
            assert_eq!(assertion.kind, AssertionKind::StartLine);
        } else {
            assert!(false, "Expected Primitive::Assertion with kind StartLine");
        }
    }
}

#[test]
fn test_parse_primitive_end_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "$",
    };

    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(ast) = result {
        if let Primitive::Assertion(assertion) = ast {
            assert_eq!(assertion.kind, AssertionKind::EndLine);
        } else {
            assert!(false, "Expected Primitive::Assertion with kind EndLine");
        }
    }
}

#[test]
fn test_parse_primitive_literal() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a",
    };

    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(ast) = result {
        if let Primitive::Literal(literal) = ast {
            assert_eq!(literal.c, 'a');
            assert_eq!(literal.kind, LiteralKind::Verbatim);
        } else {
            assert!(false, "Expected Primitive::Literal with c 'a'");
        }
    }
}

