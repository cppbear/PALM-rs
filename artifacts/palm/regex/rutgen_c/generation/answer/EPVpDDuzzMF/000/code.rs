// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    struct SimpleRegex(Regex);
    let regex = SimpleRegex(Regex(Exec { 
        ro: Arc::new(ExecReadOnly { /* initialize if necessary */ }), 
        cache: CachedThreadLocal::new() 
    }));
    
    let text = "hello";
    let start = 0;
    assert!(regex.0.is_match_at(text, start));
}

#[test]
fn test_is_match_at_start_one() {
    struct SimpleRegex(Regex);
    let regex = SimpleRegex(Regex(Exec { 
        ro: Arc::new(ExecReadOnly { /* initialize if necessary */ }), 
        cache: CachedThreadLocal::new() 
    }));
    
    let text = "hello";
    let start = 1;
    assert!(!regex.0.is_match_at(text, start));
}

#[test]
fn test_is_match_at_exceeding_length() {
    struct SimpleRegex(Regex);
    let regex = SimpleRegex(Regex(Exec { 
        ro: Arc::new(ExecReadOnly { /* initialize if necessary */ }), 
        cache: CachedThreadLocal::new() 
    }));

    let text = "hello";
    let start = 10; // Exceeds text length
    assert!(!regex.0.is_match_at(text, start));
}

#[test]
fn test_is_match_at_empty_string() {
    struct SimpleRegex(Regex);
    let regex = SimpleRegex(Regex(Exec { 
        ro: Arc::new(ExecReadOnly { /* initialize if necessary */ }), 
        cache: CachedThreadLocal::new() 
    }));

    let text = "";
    let start = 0;
    assert!(!regex.0.is_match_at(text, start));
}

