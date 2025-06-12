// Answer 0

#[test]
fn test_octal_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(false);
}

#[test]
fn test_octal_with_nest_limit_zero() {
    let mut parser_builder = ParserBuilder::new().nest_limit(0);
    parser_builder.octal(true);
}

#[test]
fn test_octal_with_nest_limit_five() {
    let mut parser_builder = ParserBuilder::new().nest_limit(5);
    parser_builder.octal(false);
}

#[test]
fn test_octal_with_nest_limit_ten() {
    let mut parser_builder = ParserBuilder::new().nest_limit(10);
    parser_builder.octal(true);
}

#[test]
fn test_octal_chain_with_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true).nest_limit(3);
}

#[test]
fn test_octal_chain_with_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(false).nest_limit(7);
}

