// Answer 0

#[test]
fn test_parse_with_comments_success() {
    let mut parser = Parser::new();
    let pattern = r#"(foo) #(this is a comment)"#;
    
    match parser.parse_with_comments(pattern) {
        Ok(with_comments) => {
            assert_eq!(with_comments.comments.len(), 1);
            assert_eq!(with_comments.comments[0].comment, "this is a comment");
        },
        Err(_) => panic!("Expected successful parsing, but received an error."),
    }
}

#[test]
fn test_parse_with_empty_comments() {
    let mut parser = Parser::new();
    let pattern = r#"(foo)"#;

    match parser.parse_with_comments(pattern) {
        Ok(with_comments) => {
            assert_eq!(with_comments.comments.len(), 0);
        },
        Err(_) => panic!("Expected successful parsing, but received an error."),
    }
}

#[test]
fn test_parse_with_nested_comments() {
    let mut parser = Parser::new();
    let pattern = r#"(foo)#(bar) #(final comment)"#;
    
    match parser.parse_with_comments(pattern) {
        Ok(with_comments) => {
            assert_eq!(with_comments.comments.len(), 2);
            assert_eq!(with_comments.comments[0].comment, "bar");
            assert_eq!(with_comments.comments[1].comment, "final comment");
        },
        Err(_) => panic!("Expected successful parsing, but received an error."),
    }
}

#[should_panic(expected = "error expected message")] // Replace with the actual panic message expected from parse_with_comments failure
#[test]
fn test_parse_with_invalid_pattern() {
    let mut parser = Parser::new();
    let pattern = r#"(foo(#"#; // Intentionally invalid to trigger an error

    parser.parse_with_comments(pattern).expect("Expected an error to be thrown.");
}

