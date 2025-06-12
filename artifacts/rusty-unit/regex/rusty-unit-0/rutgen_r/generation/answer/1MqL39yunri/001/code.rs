// Answer 0

#[test]
fn test_new_function() {
    struct MyParser<'s> {
        data: &'s str,
    }

    struct NestLimiter<'p, 's, P> {
        p: &'p P,
        depth: usize,
    }

    impl<'s> MyParser<'s> {
        fn new(data: &'s str) -> Self {
            MyParser { data }
        }
    }

    let parser = MyParser::new("test");
    let nest_limiter = NestLimiter::new(&parser);
    
    assert_eq!(nest_limiter.depth, 0);
    assert_eq!(nest_limiter.p.data, parser.data);
}

