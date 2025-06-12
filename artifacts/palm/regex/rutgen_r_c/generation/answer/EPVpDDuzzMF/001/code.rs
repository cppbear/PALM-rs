// Answer 0

#[test]
fn test_is_match_at_valid_match() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text = "abc123";
    let start = 3; // Match starts at index 3 ('1')
    assert!(regex.is_match_at(text, start));
}

#[test]
fn test_is_match_at_no_match() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text = "abc123";
    let start = 0; // No match for 'xyz'
    assert!(!regex.is_match_at(text, start));
}

#[test]
fn test_is_match_at_boundary_start() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text = "abcdef";
    let start = 0; // Match should only occur at the start
    assert!(regex.is_match_at(text, start)); // Assuming 'a' is a match
}

#[test]
fn test_is_match_at_empty_string() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text = "";
    let start = 0; // Should return false for any expected match
    assert!(!regex.is_match_at(text, start));
}

#[test]
fn test_is_match_at_out_of_bounds() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text = "abc";
    let start = 5; // Out of bounds start
    assert!(!regex.is_match_at(text, start)); // Should safely return false
}

