// Answer 0

#[test]
fn test_is_match_at_empty_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"";
    let start = 0;
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_single_element() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"a";
    let start = 0;
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_multiple_elements() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"abcde";
    let start = 0;
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_start_at_one() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"abcde";
    let start = 1;
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_start_at_end() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"abcde";
    let start = 5;
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_out_of_bounds() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: &[u8] = b"abcde";
    let start = 6; // out of bounds
    let _ = regex.is_match_at(text, start);
}

#[test]
fn test_is_match_at_large_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let text: Vec<u8> = vec![b'a'; 10_000_000]; // simulate a large text
    let start = 0;
    let _ = regex.is_match_at(&text, start);
}

