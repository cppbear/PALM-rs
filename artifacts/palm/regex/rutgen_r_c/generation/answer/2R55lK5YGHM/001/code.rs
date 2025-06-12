// Answer 0


#[test]
fn test_ignore_whitespace_enable() {
    let mut builder = ParserBuilder::new();
    let result = builder.ignore_whitespace(true);
    assert_eq!(result.ignore_whitespace, true);
}

#[test]
fn test_ignore_whitespace_disable() {
    let mut builder = ParserBuilder::new().ignore_whitespace(true);
    let result = builder.ignore_whitespace(false);
    assert_eq!(result.ignore_whitespace, false);
}

#[test]
fn test_ignore_whitespace_multiple_calls() {
    let mut builder = ParserBuilder::new();
    let result_first = builder.ignore_whitespace(true);
    let result_second = result_first.ignore_whitespace(false);
    
    assert_eq!(result_first.ignore_whitespace, true);
    assert_eq!(result_second.ignore_whitespace, false);
}

#[test]
fn test_ignore_whitespace_initial_state() {
    let builder = ParserBuilder::new();
    assert_eq!(builder.ignore_whitespace, false);
}


