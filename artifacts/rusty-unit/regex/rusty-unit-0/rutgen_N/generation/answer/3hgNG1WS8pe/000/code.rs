// Answer 0

#[test]
fn test_new_parser_builder() {
    struct ParserBuilder {
        ignore_whitespace: bool,
        nest_limit: usize,
        octal: bool,
    }

    fn new() -> ParserBuilder {
        ParserBuilder {
            ignore_whitespace: false,
            nest_limit: 250,
            octal: false,
        }
    }

    let builder = new();
    assert!(!builder.ignore_whitespace);
    assert_eq!(builder.nest_limit, 250);
    assert!(!builder.octal);
}

