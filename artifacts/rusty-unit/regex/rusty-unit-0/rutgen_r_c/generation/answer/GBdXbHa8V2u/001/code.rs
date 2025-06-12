// Answer 0

#[test]
fn test_nest_limit_set_positive() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(5);
    assert_eq!(result.nest_limit, 5);
}

#[test]
fn test_nest_limit_set_zero() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(0);
    assert_eq!(result.nest_limit, 0);
}

#[test]
fn test_nest_limit_set_large() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(u32::MAX);
    assert_eq!(result.nest_limit, u32::MAX);
}

