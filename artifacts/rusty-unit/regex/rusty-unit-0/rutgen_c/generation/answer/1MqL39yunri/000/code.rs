// Answer 0

#[test]
fn test_nest_limiter_new() {
    struct MockParser;
    
    let mock_parser = MockParser;
    let pattern = "^(a|b)$";
    
    let parser_i = ParserI {
        parser: &mock_parser,
        pattern: pattern,
    };

    let nest_limiter = NestLimiter::new(&parser_i);
    
    assert_eq!(nest_limiter.depth, 0);
    assert_eq!(nest_limiter.p.pattern, pattern);
}

