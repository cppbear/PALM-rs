// Answer 0

#[test]
fn test_nest_limiter_new() {
    struct TestParser;

    let pattern = "a*b+";
    let parser_instance = ParserI {
        parser: TestParser,
        pattern: pattern,
    };

    let nest_limiter = NestLimiter::new(&parser_instance);
    
    assert_eq!(nest_limiter.depth, 0);
    assert!(std::ptr::eq(nest_limiter.p, &parser_instance));
}

#[test]
fn test_nest_limiter_new_multiple_instances() {
    struct TestParser;

    let pattern1 = "a*b+";
    let pattern2 = "c*";
    
    let parser_instance1 = ParserI {
        parser: TestParser,
        pattern: pattern1,
    };
    
    let parser_instance2 = ParserI {
        parser: TestParser,
        pattern: pattern2,
    };

    let nest_limiter1 = NestLimiter::new(&parser_instance1);
    let nest_limiter2 = NestLimiter::new(&parser_instance2);
    
    assert_eq!(nest_limiter1.depth, 0);
    assert_eq!(nest_limiter2.depth, 0);
    assert!(!std::ptr::eq(nest_limiter1.p, nest_limiter2.p));
}

