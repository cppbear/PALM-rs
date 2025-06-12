// Answer 0

#[test]
fn test_visit_post_empty_ast() {
    let mut limiter = NestLimiter {
        p: &ParserI {
            parser: Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            },
            pattern: "",
        },
        depth: 1,
    };
    let result = limiter.visit_post(&Ast::Empty(Span::default()));
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 1);
}

#[test]
fn test_visit_post_flags_ast() {
    let mut limiter = NestLimiter {
        p: &ParserI {
            parser: Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            },
            pattern: "",
        },
        depth: 1,
    };
    let result = limiter.visit_post(&Ast::Flags(ast::SetFlags::default()));
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 1);
}

#[test]
fn test_visit_post_literal_ast() {
    let mut limiter = NestLimiter {
        p: &ParserI {
            parser: Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            },
            pattern: "",
        },
        depth: 1,
    };
    let result = limiter.visit_post(&Ast::Literal(ast::Literal::default()));
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 1);
}

#[test]
fn test_visit_post_group_ast() {
    let mut limiter = NestLimiter {
        p: &ParserI {
            parser: Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            },
            pattern: "",
        },
        depth: 1,
    };
    let result = limiter.visit_post(&Ast::Group(ast::Group::default()));
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 0); 
}

#[test]
fn test_visit_post_repetition_ast() {
    let mut limiter = NestLimiter {
        p: &ParserI {
            parser: Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            },
            pattern: "",
        },
        depth: 1,
    };
    let result = limiter.visit_post(&Ast::Repetition(ast::Repetition::default()));
    assert!(result.is_ok());
    assert_eq!(limiter.depth, 0);
}

