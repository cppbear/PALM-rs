// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\d" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(!result.negated);
    assert_eq!(result.span, span);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\D" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(result.negated);
    assert_eq!(result.span, span);
}

#[test]
fn test_parse_perl_class_space() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\s" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(!result.negated);
    assert_eq!(result.span, span);
}

#[test]
fn test_parse_perl_class_negated_space() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\S" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(result.negated);
    assert_eq!(result.span, span);
}

#[test]
fn test_parse_perl_class_word() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\w" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(!result.negated);
    assert_eq!(result.span, span);
}

#[test]
fn test_parse_perl_class_negated_word() {
    let span = Span { start: Position(0), end: Position(1) };
    let parser = ParserI { parser: Parser { pos: Cell::new(Position(1)), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "\\W" };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(result.negated);
    assert_eq!(result.span, span);
}

