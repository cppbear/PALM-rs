// Answer 0

#[test]
fn test_new_parser_with_valid_pattern() {
    struct DummyParser;

    let pattern = "a*b+";
    let parser = DummyParser;
    let parser_instance = new(parser, pattern);
    
    assert_eq!(parser_instance.pattern, pattern);
}

#[test]
fn test_new_parser_with_empty_pattern() {
    struct DummyParser;

    let pattern = "";
    let parser = DummyParser;
    let parser_instance = new(parser, pattern);
    
    assert_eq!(parser_instance.pattern, pattern);
}

