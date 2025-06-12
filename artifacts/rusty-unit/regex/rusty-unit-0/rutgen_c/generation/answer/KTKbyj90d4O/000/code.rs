// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<group>";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser_instance = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern };

    let result = parser_instance.parse_capture_name(0);
    assert!(result.is_ok());
    let capname = result.unwrap();
    assert_eq!(capname.name, "group");
    assert_eq!(capname.index, 0);
}

#[test]
fn test_parse_capture_name_empty() {
    let pattern = "<>";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser_instance = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern };

    let result = parser_instance.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::GroupNameEmpty);
}

#[test]
fn test_parse_capture_name_unclosed() {
    let pattern = "<group";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser_instance = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern };

    let result = parser_instance.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::GroupNameUnexpectedEof);
}

#[test]
fn test_parse_capture_name_invalid_char() {
    let pattern = "<group#>";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser_instance = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern };

    let result = parser_instance.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::GroupNameInvalid);
}

#[test]
fn test_parse_capture_name_eof() {
    let pattern = "<group>";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser_instance = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern };

    parser_instance.parser.pos.set(Position { offset: 8, line: 1, column: 9 }); // Move position to EOF
    let result = parser_instance.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::GroupNameUnexpectedEof);
}

