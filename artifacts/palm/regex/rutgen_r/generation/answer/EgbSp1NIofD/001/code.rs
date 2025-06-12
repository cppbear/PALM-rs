// Answer 0

#[derive(Debug)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

struct Parser {
    pos: Cell<Position>,
    capture_index: Cell<usize>,
    nest_limit: usize,
    octal: bool,
    initial_ignore_whitespace: bool,
    ignore_whitespace: Cell<bool>,
    comments: RefCell<Vec<String>>,
    stack_group: RefCell<Vec<String>>,
    stack_class: RefCell<Vec<String>>,
    capture_names: RefCell<Vec<String>>,
    scratch: RefCell<String>,
}

struct Config {
    nest_limit: usize,
    octal: bool,
    ignore_whitespace: bool,
}

impl Config {
    pub fn build(&self) -> Parser {
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

#[test]
fn test_build_parser_default() {
    let config = Config {
        nest_limit: 100,
        octal: false,
        ignore_whitespace: false,
    };
    let parser = config.build();
    
    assert_eq!(parser.pos.get().offset, 0);
    assert_eq!(parser.pos.get().line, 1);
    assert_eq!(parser.pos.get().column, 1);
    assert_eq!(parser.capture_index.get(), 0);
    assert_eq!(parser.nest_limit, 100);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, false);
    assert_eq!(parser.ignore_whitespace.get(), false);
    assert!(parser.comments.borrow().is_empty());
    assert!(parser.stack_group.borrow().is_empty());
    assert!(parser.stack_class.borrow().is_empty());
    assert!(parser.capture_names.borrow().is_empty());
    assert!(parser.scratch.borrow().is_empty());
}

#[test]
fn test_build_parser_with_octal() {
    let config = Config {
        nest_limit: 50,
        octal: true,
        ignore_whitespace: true,
    };
    let parser = config.build();

    assert_eq!(parser.nest_limit, 50);
    assert!(parser.octal);
    assert!(parser.initial_ignore_whitespace);
}

#[test]
fn test_build_parser_large_nest_limit() {
    let config = Config {
        nest_limit: usize::MAX,
        octal: false,
        ignore_whitespace: true,
    };
    let parser = config.build();

    assert_eq!(parser.nest_limit, usize::MAX);
}

