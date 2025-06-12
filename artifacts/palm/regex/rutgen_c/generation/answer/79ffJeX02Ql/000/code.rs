// Answer 0

#[test]
fn test_approximate_size_empty_patterns() {
    let pats: Vec<Vec<u8>> = Vec::new();
    let literals = Literals::new(pats.clone()).unwrap();
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.approximate_size(), 0);
}

#[test]
fn test_approximate_size_single_pattern() {
    let pats: Vec<Vec<u8>> = vec![b"hello".to_vec()];
    let literals = Literals::new(pats.clone()).unwrap();
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.approximate_size(), 5);
}

#[test]
fn test_approximate_size_multiple_patterns() {
    let pats: Vec<Vec<u8>> = vec![b"hello".to_vec(), b"world".to_vec(), b"rust".to_vec()];
    let literals = Literals::new(pats.clone()).unwrap();
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.approximate_size(), 5 + 5 + 4); // 5 + 5 + 4 = 14
}

#[test]
fn test_approximate_size_patterns_with_empty_strings() {
    let pats: Vec<Vec<u8>> = vec![b"".to_vec(), b"hello".to_vec()];
    let literals = Literals::new(pats.clone()).unwrap();
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.approximate_size(), 0 + 5); // 0 + 5 = 5
}

