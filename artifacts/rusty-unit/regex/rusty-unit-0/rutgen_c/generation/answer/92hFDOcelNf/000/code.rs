// Answer 0

#[test]
fn test_increment_depth_within_limit() {
    #[derive(Borrow)]
    struct DummyParser;
    
    impl Parser for DummyParser {}

    let mut parser_instance = ParserI::new(DummyParser, "test_pattern");
    let mut nest_limiter = NestLimiter::new(&parser_instance);
    nest_limiter.depth = 0;
    parser_instance.parser().nest_limit = 5;

    let span = Span { start: 0, end: 1 };
    
    let result = nest_limiter.increment_depth(&span);
    assert!(result.is_ok());
    assert_eq!(nest_limiter.depth, 1);
}

#[test]
fn test_increment_depth_exceeds_limit() {
    #[derive(Borrow)]
    struct DummyParser;

    impl Parser for DummyParser {}

    let mut parser_instance = ParserI::new(DummyParser, "test_pattern");
    let mut nest_limiter = NestLimiter::new(&parser_instance);
    nest_limiter.depth = 5;
    parser_instance.parser().nest_limit = 5;
    
    let span = Span { start: 0, end: 1 };

    let result = nest_limiter.increment_depth(&span);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_increment_depth_overflows() {
    #[derive(Borrow)]
    struct DummyParser;

    impl Parser for DummyParser {}

    let mut parser_instance = ParserI::new(DummyParser, "test_pattern");
    let mut nest_limiter = NestLimiter::new(&parser_instance);
    nest_limiter.depth = u32::MAX;

    let span = Span { start: 0, end: 1 };

    let _ = nest_limiter.increment_depth(&span);
}

