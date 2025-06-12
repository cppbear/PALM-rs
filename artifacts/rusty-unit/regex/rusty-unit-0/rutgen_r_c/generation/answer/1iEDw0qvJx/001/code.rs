// Answer 0

#[test]
fn test_parser_new() {
    // Test the creation of a new Parser with default configuration
    let parser = Parser::new();
    
    // Check that the parser has been initialized with default values
    // These assertions will require you to know the default values set in the Parser struct.
    
    assert_eq!(parser.nest_limit, 256); // Example default value, replace with actual value
    assert_eq!(parser.octal, false); // Example default value, replace with actual value
    assert!(parser.ignore_whitespace.get()); // Example default value, replace with actual value
}

#[test]
fn test_parser_builder_new() {
    // Test the creation of a new ParserBuilder with default configuration
    let builder = ParserBuilder::new();
    
    // Ensure that the builder has its initial values set correctly
    assert_eq!(builder.nest_limit, 256); // Example default value, replace with actual value
    assert_eq!(builder.octal, false); // Example default value, replace with actual value
}

#[test]
fn test_parser_builder_nest_limit() {
    // Test setting the nested limit in the ParserBuilder
    let mut builder = ParserBuilder::new();
    builder.nest_limit(100);
    
    let parser = builder.build();

    assert_eq!(parser.nest_limit, 100);
}

#[test]
fn test_parser_builder_octal() {
    // Test enabling octal syntax in the ParserBuilder
    let mut builder = ParserBuilder::new();
    builder.octal(true);

    let parser = builder.build();

    assert_eq!(parser.octal, true);
}

#[test]
fn test_parser_builder_ignore_whitespace() {
    // Test ignoring whitespace in the ParserBuilder
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);

    let parser = builder.build();

    assert!(parser.ignore_whitespace.get());
}

