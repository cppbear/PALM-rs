// Answer 0

#[test]
fn test_parse_capture_name_invalid_char() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let capture_index = 0;
    let pattern = ">";

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let char_fn = |parser: &ParserI| {
        if parser.pos.get().offset == 0 {
            'a'
        } else {
            '>'
        }
    };
    let original_char = parser.char();
    let char = char_fn(&parser);
    parser.parser.pos.set(Position { offset: 1, line: 1, column: 2 });
    
    parser.parse_capture_name(capture_index);
}

