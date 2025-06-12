// Answer 0

#[test]
fn test_parser_i_new() {
    struct MockParser;

    let pattern = "a*b";
    let parser_instance = MockParser;
    
    let parser_i = ParserI::new(&parser_instance, pattern);

    assert_eq!(parser_i.pattern, pattern);
}

#[test]
fn test_parser_i_new_empty_pattern() {
    struct MockParser;

    let pattern = "";
    let parser_instance = MockParser;
    
    let parser_i = ParserI::new(&parser_instance, pattern);

    assert_eq!(parser_i.pattern, pattern);
}

#[test]
fn test_parser_i_new_long_pattern() {
    struct MockParser;

    let pattern = "abcde*fghij";
    let parser_instance = MockParser;
    
    let parser_i = ParserI::new(&parser_instance, pattern);

    assert_eq!(parser_i.pattern, pattern);
}

