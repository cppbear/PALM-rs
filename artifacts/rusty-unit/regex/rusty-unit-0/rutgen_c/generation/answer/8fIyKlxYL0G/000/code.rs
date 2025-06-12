// Answer 0

#[test]
fn test_is_match_at_no_match() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { /* initialize fields as needed */ }), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"hello";
    let result = regex.is_match_at(text, 0);
    assert_eq!(result, false);
}

#[test]
fn test_is_match_at_match_start_0() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { /* initialize fields as needed */ }), cache: CachedThreadLocal::new() });
    // Assuming we have a regex that matches "hello"
    regex.set_pattern("hello"); // Placeholder, replace with actual method to set pattern
    let text: &[u8] = b"hello world";
    let result = regex.is_match_at(text, 0);
    assert_eq!(result, true);
}

#[test]
fn test_is_match_at_match_with_offset() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { /* initialize fields as needed */ }), cache: CachedThreadLocal::new() });
    // Assuming we have a regex that matches "world"
    regex.set_pattern("world"); // Placeholder, replace with actual method to set pattern
    let text: &[u8] = b"hello world";
    let result = regex.is_match_at(text, 6);
    assert_eq!(result, true);
}

#[test]
fn test_is_match_at_not_matching_offset() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { /* initialize fields as needed */ }), cache: CachedThreadLocal::new() });
    // Assuming we have a regex that matches "world"
    regex.set_pattern("world"); // Placeholder, replace with actual method to set pattern
    let text: &[u8] = b"hello world";
    let result = regex.is_match_at(text, 5);
    assert_eq!(result, false);
}

#[test]
fn test_is_match_at_out_of_bounds() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly { /* initialize fields as needed */ }), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"hello";
    let result = regex.is_match_at(text, 10);
    assert_eq!(result, false);
}

