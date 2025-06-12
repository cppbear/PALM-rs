// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(0);
    assert_eq!(result.nest_limit, 0);
}

#[test]
fn test_nest_limit_positive() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(5);
    assert_eq!(result.nest_limit, 5);
}

#[test]
fn test_nest_limit_edge_case() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(u32::MAX);
    assert_eq!(result.nest_limit, u32::MAX);
}

#[test]
fn test_nest_limit_chained() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(3).nest_limit(10);
    assert_eq!(result.nest_limit, 10);
}

#[test]
fn test_nest_limit_no_effect_empty() {
    let mut builder = ParserBuilder::new();
    let initial_limit = builder.nest_limit(0).nest_limit(0).nest_limit(0).nest_limit(0);
    assert_eq!(initial_limit.nest_limit, 0);
}

