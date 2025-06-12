// Answer 0

#[test]
fn test_parse_unicode_class_single_letter() {
    let pattern = "\\pN"; // Valid single character notation 
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::from("N")),
        },
        pattern: pattern,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::OneLetter('N'));
}

#[test]
fn test_parse_unicode_class_negated_single_letter() {
    let pattern = "\\pP"; // Valid negated single character notation 
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::from("P")),
        },
        pattern: pattern,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, true);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::OneLetter('P'));
}

#[test]
fn test_parse_unicode_class_bracketed() {
    let pattern = "\\p{Greek}"; // Valid bracketed notation
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::from("Greek")),
        },
        pattern: pattern,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, false);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::Named("Greek".to_string()));
}

#[test]
fn test_parse_unicode_class_negated_bracketed() {
    let pattern = "\\p{scx!=Katakana}"; // Valid negated bracketed notation
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::from("scx!=Katakana")),
        },
        pattern: pattern,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let class_unicode = result.unwrap();
    assert_eq!(class_unicode.negated, true);
    assert_eq!(class_unicode.kind, ast::ClassUnicodeKind::NamedValue {
        op: ast::ClassUnicodeOpKind::NotEqual,
        name: "scx".to_string(),
        value: "Katakana".to_string(),
    });
}

