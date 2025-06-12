// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(0);
    assert_eq!(result.nest_limit, 0);
}

#[test]
fn test_nest_limit_one() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(1);
    assert_eq!(result.nest_limit, 1);
}

#[test]
fn test_nest_limit_large() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(1000);
    assert_eq!(result.nest_limit, 1000);
}

#[test]
fn test_nest_limit_underflow() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(u32::MIN);
    assert_eq!(result.nest_limit, 0);
}

#[test]
fn test_nest_limit_boundary() {
    let mut builder = ParserBuilder::new();
    // Set limit to maximum.
    let result = builder.nest_limit(u32::MAX);
    assert_eq!(result.nest_limit, u32::MAX);
}

