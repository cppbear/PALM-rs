// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(0);
}

#[test]
fn test_nest_limit_one() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(1);
}

#[test]
fn test_nest_limit_two() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(2);
}

#[test]
fn test_nest_limit_three() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(3);
}

#[test]
fn test_nest_limit_four() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(4);
}

#[test]
fn test_nest_limit_five() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(5);
}

#[test]
fn test_nest_limit_ten() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(10);
}

#[test]
fn test_nest_limit_twenty() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(20);
}

#[test]
fn test_nest_limit_one_hundred() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(100);
}

#[test]
fn test_nest_limit_one_thousand() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.nest_limit(1000);
}

