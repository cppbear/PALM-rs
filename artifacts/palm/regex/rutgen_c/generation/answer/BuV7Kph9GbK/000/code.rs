// Answer 0

#[test]
fn test_decrement_depth_positive_depth() {
    struct TestParser;
    let parser = TestParser;
    let ast = ParserI {
        parser,
        pattern: "a*b*c*",
    };

    let mut limiter = NestLimiter::new(&ast);
    limiter.depth = 1;
    
    limiter.decrement_depth();
    
    assert_eq!(limiter.depth, 0);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_decrement_depth_zero_depth() {
    struct TestParser;
    let parser = TestParser;
    let ast = ParserI {
        parser,
        pattern: "a*b*c*",
    };

    let mut limiter = NestLimiter::new(&ast);
    limiter.depth = 0;

    limiter.decrement_depth();
}

