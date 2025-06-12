// Answer 0

#[test]
fn test_nest_limiter_finish_success() {
    struct MockParser;

    let parser_instance = MockParser;
    let pattern = "a*b+";
    let nest_limiter = NestLimiter {
        p: &parser_instance,
        depth: 0,
    };

    let result = nest_limiter.finish();
    assert_eq!(result, Ok(()));
}

