// Answer 0

#[test]
fn test_build_parser_with_default_parameters() {
    struct Config {
        nest_limit: usize,
        octal: bool,
        ignore_whitespace: bool,
    }

    impl Config {
        fn new(nest_limit: usize, octal: bool, ignore_whitespace: bool) -> Self {
            Self { nest_limit, octal, ignore_whitespace }
        }

        fn build(&self) -> Parser {
            Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
                octal: self.octal,
                initial_ignore_whitespace: self.ignore_whitespace,
                ignore_whitespace: Cell::new(self.ignore_whitespace),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let config = Config::new(5, false, true);
    let parser = config.build();
    assert_eq!(parser.nest_limit, 5);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, true);
}

#[test]
fn test_build_parser_with_non_default_parameters() {
    struct Config {
        nest_limit: usize,
        octal: bool,
        ignore_whitespace: bool,
    }

    impl Config {
        fn new(nest_limit: usize, octal: bool, ignore_whitespace: bool) -> Self {
            Self { nest_limit, octal, ignore_whitespace }
        }

        fn build(&self) -> Parser {
            Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: self.nest_limit,
                octal: self.octal,
                initial_ignore_whitespace: self.ignore_whitespace,
                ignore_whitespace: Cell::new(self.ignore_whitespace),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let config = Config::new(10, true, false);
    let parser = config.build();
    assert_eq!(parser.nest_limit, 10);
    assert_eq!(parser.octal, true);
    assert_eq!(parser.initial_ignore_whitespace, false);
}

