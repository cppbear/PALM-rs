// Answer 0

#[test]
fn test_visit_pre_with_alternation() {
    let span = Span { start: 0, end: 5 };
    let ast = Ast::Alternation(Alternation {
        span,
        asts: vec![],
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
        pattern: "a|b|c",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    
    let result = nest_limiter.visit_pre(&ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_with_repetition() {
    let span = Span { start: 0, end: 5 };
    let ast = Ast::Repetition(Repetition {
        span,
        op: RepetitionOp::Plus,
        greedy: true,
        ast: Box::new(Ast::Literal(Literal::new('a', span.clone()))),
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
        pattern: "a+",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    
    let result = nest_limiter.visit_pre(&ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_with_group() {
    let span = Span { start: 0, end: 5 };
    let ast = Ast::Group(Group {
        span,
        kind: GroupKind::Regular,
        ast: Box::new(Ast::Literal(Literal::new('a', span.clone()))),
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
        pattern: "(a)",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    
    let result = nest_limiter.visit_pre(&ast);
    
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_exceed_nest_limit() {
    let span = Span { start: 0, end: 5 };
    let ast = Ast::Alternation(Alternation {
        span,
        asts: vec![],
    });

    let parser = ParserI {
        parser: Parser { 
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 1,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "a|b|c",
    };

    let mut nest_limiter = NestLimiter::new(&parser);
    
    // First call should succeed
    let first_result = nest_limiter.visit_pre(&ast);
    assert!(first_result.is_ok());

    // Simulate one more depth
    nest_limiter.increment_depth(&span).unwrap();

    // Second call should exceed limit and return an error
    let second_result = nest_limiter.visit_pre(&ast);
    assert!(second_result.is_err());
}

