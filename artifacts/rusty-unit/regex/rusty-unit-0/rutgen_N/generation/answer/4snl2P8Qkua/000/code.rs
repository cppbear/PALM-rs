// Answer 0

#[test]
fn test_nest_limit_zero() {
    struct ParserBuilder {
        nest_limit: u32,
    }
    
    let mut builder = ParserBuilder { nest_limit: 0 };
    builder.nest_limit(0);
    assert_eq!(builder.nest_limit, 0);
}

#[test]
fn test_nest_limit_positive() {
    struct ParserBuilder {
        nest_limit: u32,
    }
    
    let mut builder = ParserBuilder { nest_limit: 5 };
    builder.nest_limit(3);
    assert_eq!(builder.nest_limit, 3);
}

#[test]
fn test_nest_limit_large_value() {
    struct ParserBuilder {
        nest_limit: u32,
    }
    
    let mut builder = ParserBuilder { nest_limit: 10 };
    builder.nest_limit(u32::MAX);
    assert_eq!(builder.nest_limit, u32::MAX);
}

