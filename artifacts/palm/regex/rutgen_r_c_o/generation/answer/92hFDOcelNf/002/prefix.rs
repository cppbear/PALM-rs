// Answer 0

#[test]
fn test_increment_depth_panic_due_to_exceeding_nest_limit() {
    let depth = 4294967295;
    let nest_limit = 4294967295;
    let span = Span { start: Position::default(), end: Position::default() };

    let parser_instance = ParserI::new(
        Parser {
            nest_limit,
            ..Default::default()
        },
        "pattern"
    );

    let mut nest_limiter = NestLimiter {
        p: &parser_instance,
        depth,
    };

    let result = nest_limiter.increment_depth(&span);
}

#[test]
fn test_increment_depth_valid_increment() {
    let depth = 1;
    let nest_limit = 4294967295;
    let span = Span { start: Position::default(), end: Position::default() };

    let parser_instance = ParserI::new(
        Parser {
            nest_limit,
            ..Default::default()
        },
        "pattern"
    );

    let mut nest_limiter = NestLimiter {
        p: &parser_instance,
        depth,
    };

    let result = nest_limiter.increment_depth(&span);
}

#[test]
fn test_increment_depth_at_limit() {
    let depth = 4294967294;
    let nest_limit = 4294967295;
    let span = Span { start: Position::default(), end: Position::default() };

    let parser_instance = ParserI::new(
        Parser {
            nest_limit,
            ..Default::default()
        },
        "pattern"
    );

    let mut nest_limiter = NestLimiter {
        p: &parser_instance,
        depth,
    };

    let result = nest_limiter.increment_depth(&span);
}

