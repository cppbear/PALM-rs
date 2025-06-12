// Answer 0

#[test]
#[should_panic]
fn test_parse_counted_repetition_char_not_open_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a{2}" };
    
    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_no_expr() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "{2}" };
    
    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_bump_and_bump_space_false() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "{2}" };
    
    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_eof_reached() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "{2,}" };
    
    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_character_is_not_comma() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "{2,2}" };
    
    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_bump_and_bump_space_false_second_time() {
    let position = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(position, position);
    let concat = Concat { span, asts: vec![Ast::Empty(span.clone())] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "{2,}" };

    parser.parse_counted_repetition(concat);
}

