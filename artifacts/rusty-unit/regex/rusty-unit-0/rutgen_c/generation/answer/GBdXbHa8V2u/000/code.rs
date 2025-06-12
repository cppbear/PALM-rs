// Answer 0

#[test]
fn test_nest_limit_set_positive_value() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(5);
    assert_eq!(result.ast.nest_limit, 5);
}

#[test]
fn test_nest_limit_set_zero() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(0);
    // Additional checks can be included if the ParserBuilder has a method to fetch the limit.
    // Here it's assumed the limit is stored in the ast field.
    assert_eq!(builder.ast.nest_limit, 0);
}

#[test]
fn test_nest_limit_set_large_value() {
    let mut builder = ParserBuilder::new();
    let result = builder.nest_limit(100);
    assert_eq!(result.ast.nest_limit, 100);
}

#[test]
fn test_nest_limit_set_default_value() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(1);
    assert_eq!(builder.ast.nest_limit, 1);
}

