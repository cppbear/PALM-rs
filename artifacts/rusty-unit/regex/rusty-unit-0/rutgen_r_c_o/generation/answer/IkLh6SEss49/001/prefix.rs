// Answer 0

#[test]
fn test_read_captures_at_empty_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: &[u8] = b"";
    let start = 0;
    regex.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_short_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: &[u8] = b"abc";
    let start = 0;
    regex.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_full_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: &[u8] = b"abcdefghij"; // Length 10
    let start = 5; // arbitrary start within text length
    regex.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_edge_case_start_zero() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: &[u8] = b"abc";
    let start = 0;
    regex.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_edge_case_start_length() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: &[u8] = b"abc";
    let start = text.len(); // start at length of text
    regex.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_large_text() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(Vec::new());
    let text: Vec<u8> = (0..1_000_000).map(|x| x as u8).collect(); // 1 million bytes
    let start = 999_999; // near end boundary
    regex.read_captures_at(&mut locs, &text, start);
}

#[test]
fn test_read_captures_at_large_loc_size() {
    let regex = Regex(Exec { ro: Arc::new(ExecReadOnly::new()), cache: CachedThreadLocal::new() });
    let mut locs = Locations(vec![Slot::default(); 1000]); // max locs size
    let text: &[u8] = b"abc";
    let start = 1; 
    regex.read_captures_at(&mut locs, text, start);
}

