// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(0);
}

#[test]
fn test_nest_limit_one() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(1);
}

#[test]
fn test_nest_limit_five() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(5);
}

#[test]
fn test_nest_limit_ten() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(10);
}

#[test]
fn test_nest_limit_boundary_values() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(999);
    builder.nest_limit(1000);
}

