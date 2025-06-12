// Answer 0

#[test]
fn test_find_at_valid_match() {
    struct TestRegex(Exec);
    
    let exec = Exec {
        ro: Arc::new(ExecReadOnly::new()), // Assuming an appropriate initialization method
        cache: CachedThreadLocal::new(), // Assuming an appropriate initialization method
    };
    let regex = TestRegex(exec);

    let text = b"hello, world!";
    let start = 0;
    let result = regex.find_at(text, start);

    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.text, b"hello, world!");
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 5); // Assuming the regex matches "hello"
    }
}

#[test]
fn test_find_at_no_match() {
    struct TestRegex(Exec);
    
    let exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let regex = TestRegex(exec);

    let text = b"hello, world!";
    let start = 6; // Starting point where no match occurs
    let result = regex.find_at(text, start);
    
    assert!(result.is_none());
}

#[test]
fn test_find_at_partial_match() {
    struct TestRegex(Exec);
    
    let exec = Exec {
        ro: Arc::new(ExecReadOnly::new()),
        cache: CachedThreadLocal::new(),
    };
    let regex = TestRegex(exec);

    let text = b"hello, world!";
    let start = 5; // Starting after "hello"
    let result = regex.find_at(text, start);

    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.text, b"hello, world!");
        assert_eq!(m.start, 7); // Assuming it matches "world"
        assert_eq!(m.end, 12);
    }
}

