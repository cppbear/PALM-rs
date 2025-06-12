// Answer 0

#[test]
fn test_visit_pre_group() {
    let ast_group = ast::Ast::Group(ast::Group {
        span: ast::Span {
            start: 0,
            end: 5,
        },
        kind: ast::GroupKind::Capture,
        ast: Box::new(ast::Ast::Literal(ast::Literal::new("abc", 0))),
    });
    
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
        pattern: "abc",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    let result = nest_limiter.visit_pre(&ast_group);
    
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_repetition() {
    let ast_repetition = ast::Ast::Repetition(ast::Repetition {
        span: ast::Span {
            start: 0,
            end: 5,
        },
        op: ast::RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(ast::Ast::Literal(ast::Literal::new("a", 0))),
    });

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
        pattern: "a*",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    let result = nest_limiter.visit_pre(&ast_repetition);

    assert!(result.is_ok());
} 

#[test]
fn test_visit_pre_alternation() {
    let ast_alternation = ast::Ast::Alternation(ast::Alternation {
        span: ast::Span {
            start: 0,
            end: 5,
        },
        asts: vec![
            ast::Ast::Literal(ast::Literal::new("a", 0)),
            ast::Ast::Literal(ast::Literal::new("b", 1)),
        ],
    });

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
        pattern: "a|b",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    let result = nest_limiter.visit_pre(&ast_alternation);

    assert!(result.is_ok());
} 

#[test]
fn test_visit_pre_concat() {
    let ast_concat = ast::Ast::Concat(ast::Concat {
        span: ast::Span {
            start: 0,
            end: 3,
        },
        asts: vec![ast::Ast::Literal(ast::Literal::new("ab", 0))],
    });

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
        pattern: "ab",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    let result = nest_limiter.visit_pre(&ast_concat);

    assert!(result.is_ok());
} 

#[test]
#[should_panic]
fn test_visit_pre_nest_limit_exceeded() {
    let ast_group = ast::Ast::Group(ast::Group {
        span: ast::Span {
            start: 0,
            end: 5,
        },
        kind: ast::GroupKind::Capture,
        ast: Box::new(ast::Ast::Literal(ast::Literal::new("abc", 0))),
    });

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 1, // Set nest limit to 1 to exceed it
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "abc",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    nest_limiter.increment_depth(&ast_group.span);
    nest_limiter.visit_pre(&ast_group).unwrap();
}

