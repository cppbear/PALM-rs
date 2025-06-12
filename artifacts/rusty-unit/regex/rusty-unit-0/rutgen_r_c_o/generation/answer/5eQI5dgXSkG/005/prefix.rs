// Answer 0

#[test]
fn test_parse_perl_class_space() {
    struct TestParser {
        pos: Cell<Position>,
        capture_index: Cell<u32>,
        nest_limit: u32,
        octal: bool,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl TestParser {
        fn char(&self) -> char {
            's'
        }

        fn span_char(&self) -> Span {
            Span { start: Position(0), end: Position(1) }
        }

        fn bump(&self) {}
    }

    let parser = TestParser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        ignore_whitespace: Cell::new(false),
        pattern: String::from("\\s"),
    };

    let parser_instance = ParserI {
        parser: &parser,
        pattern: &parser.pattern,
    };

    parser_instance.parse_perl_class();
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct TestParser {
        pos: Cell<Position>,
        capture_index: Cell<u32>,
        nest_limit: u32,
        octal: bool,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl TestParser {
        fn char(&self) -> char {
            'S'
        }

        fn span_char(&self) -> Span {
            Span { start: Position(0), end: Position(1) }
        }

        fn bump(&self) {}
    }

    let parser = TestParser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        ignore_whitespace: Cell::new(false),
        pattern: String::from("\\S"),
    };

    let parser_instance = ParserI {
        parser: &parser,
        pattern: &parser.pattern,
    };

    parser_instance.parse_perl_class();
}

