// Answer 0

#[test]
fn test_extensions_new() {
    let extensions = Extensions::new();
    assert!(extensions.map.is_none());
}

#[test]
fn test_extensions_is_empty() {
    let mut extensions = Extensions::new();
    assert!(extensions.is_empty());
}

#[test]
fn test_extensions_len() {
    let extensions = Extensions::new();
    assert_eq!(extensions.len(), 0);
}

#[test]
fn test_extensions_clear() {
    let mut extensions = Extensions::new();
    extensions.clear();
    assert!(extensions.is_empty());
}

