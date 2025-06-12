// Answer 0

#[test]
fn test_increment_depth_at_limit() {
    let span = Span { start: 0, end: 1 };
    let limit = 10;
    let parser = ParserI {
        parser: Box::new(Parser { nest_limit: limit }),
        pattern: "test_pattern",
    };
    let mut nest_limiter = NestLimiter { p: &parser, depth: 9 };
    nest_limiter.increment_depth(&span);
}

#[test]
fn test_increment_depth_non_panic() {
    let span = Span { start: 0, end: 1 };
    let limit = 10;
    let parser = ParserI {
        parser: Box::new(Parser { nest_limit: limit }),
        pattern: "test_pattern",
    };
    let mut nest_limiter = NestLimiter { p: &parser, depth: 8 };
    nest_limiter.increment_depth(&span);
}

#[test]
#[should_panic]
fn test_increment_depth_panic_overflow() {
    let span = Span { start: 0, end: 1 };
    let limit = 1;
    let parser = ParserI {
        parser: Box::new(Parser { nest_limit: limit }),
        pattern: "test_pattern",
    };
    let mut nest_limiter = NestLimiter { p: &parser, depth: 9 };
    nest_limiter.increment_depth(&span);
}

#[test]
#[should_panic]
fn test_increment_depth_exceed_limit() {
    let span = Span { start: 0, end: 1 };
    let limit = 5;
    let parser = ParserI {
        parser: Box::new(Parser { nest_limit: limit }),
        pattern: "test_pattern",
    };
    let mut nest_limiter = NestLimiter { p: &parser, depth: 5 };
    nest_limiter.increment_depth(&span);
}

