// Answer 0

#[test]
fn test_find_with_empty_haystack() {
    let pats = Literals::new(&[]).unwrap();
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(&[]);
    assert!(result.is_none());
}

#[test]
fn test_find_with_non_matching_haystack() {
    let pats = Literals::new(&[b"abc", b"def"]).unwrap();
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(b"xyz");
    assert!(result.is_none());
}

#[test]
fn test_find_with_matching_haystack() {
    let pats = Literals::new(&[b"abc", b"def"]).unwrap();
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(b"123abc456");
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.pat, 0);
        assert_eq!(m.start, 3);
        assert_eq!(m.end, 6);
    }
}

#[test]
fn test_find_with_multiple_matches() {
    let pats = Literals::new(&[b"abc", b"def", b"abc"]).unwrap();
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(b"abc456abc");
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.pat, 0);
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 3);
    }
}

#[test]
fn test_find_with_matches_at_boundary() {
    let pats = Literals::new(&[b"abc", b"def"]).unwrap();
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(b"abctest");
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.pat, 0);
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 3);
    }
}

