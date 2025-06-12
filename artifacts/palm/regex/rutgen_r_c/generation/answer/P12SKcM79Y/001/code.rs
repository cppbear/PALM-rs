// Answer 0

#[test]
fn test_build_parser_with_default_settings() {
    let builder = ParserBuilder::new()
        .nest_limit(256)
        .octal(false)
        .ignore_whitespace(false);
        
    let parser = builder.build();
    
    assert_eq!(parser.nest_limit, 256);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, false);
}

#[test]
fn test_build_parser_with_octal_enabled() {
    let builder = ParserBuilder::new()
        .nest_limit(128)
        .octal(true)
        .ignore_whitespace(false);
        
    let parser = builder.build();
    
    assert_eq!(parser.nest_limit, 128);
    assert_eq!(parser.octal, true);
    assert_eq!(parser.initial_ignore_whitespace, false);
}

#[test]
fn test_build_parser_with_whitespace_ignored() {
    let builder = ParserBuilder::new()
        .nest_limit(100)
        .octal(false)
        .ignore_whitespace(true);
        
    let parser = builder.build();
    
    assert_eq!(parser.nest_limit, 100);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, true);
}

#[test]
fn test_build_parser_with_high_nest_limit() {
    let builder = ParserBuilder::new()
        .nest_limit(u32::MAX)
        .octal(false)
        .ignore_whitespace(true);
        
    let parser = builder.build();
    
    assert_eq!(parser.nest_limit, u32::MAX);
    assert_eq!(parser.octal, false);
    assert_eq!(parser.initial_ignore_whitespace, true);
} 

#[should_panic]
fn test_build_parser_with_invalid_nest_limit() {
    let builder = ParserBuilder::new().nest_limit(0);
    builder.build();
}

