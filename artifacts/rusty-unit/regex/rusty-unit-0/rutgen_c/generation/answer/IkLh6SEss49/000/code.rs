// Answer 0

#[test]
fn test_read_captures_at_empty_text() {
    let exec_ro = Arc::new(ExecReadOnly::new());
    let exec = Exec {
        ro: exec_ro.clone(),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let text: &[u8] = b"";
    let mut locs = Locations(vec![]);
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_non_matching_text() {
    let exec_ro = Arc::new(ExecReadOnly::new());
    let exec = Exec {
        ro: exec_ro.clone(),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let text: &[u8] = b"hello world";
    let mut locs = Locations(vec![]);
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_matching_text() {
    let exec_ro = Arc::new(ExecReadOnly::new());
    let exec = Exec {
        ro: exec_ro.clone(),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let text: &[u8] = b"hello 123";
    let mut locs = Locations(vec![]);
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.text, b"hello 123");
        assert!(m.start < m.end);
    }
}

#[test]
fn test_read_captures_at_start_offset() {
    let exec_ro = Arc::new(ExecReadOnly::new());
    let exec = Exec {
        ro: exec_ro.clone(),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    let text: &[u8] = b"example 456 and more";
    let mut locs = Locations(vec![]);
    let result = regex.read_captures_at(&mut locs, text, 8);
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.text, b"456");
        assert!(m.start < m.end);
    }
}

