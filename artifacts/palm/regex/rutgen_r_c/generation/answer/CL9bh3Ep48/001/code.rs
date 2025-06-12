// Answer 0

#[test]
fn test_teddy_find_empty_haystack() {
    let patterns: Vec<Vec<u8>> = vec![b"test".to_vec()];
    let literals = Literals::new(&patterns);
    let teddy = Teddy::new(&literals).unwrap();
    let result = teddy.find(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_teddy_find_non_matching_haystack() {
    let patterns: Vec<Vec<u8>> = vec![b"test".to_vec(), b"pattern".to_vec()];
    let literals = Literals::new(&patterns);
    let teddy = Teddy::new(&literals).unwrap();
    let result = teddy.find(b"no matches here");
    assert_eq!(result, None);
}

#[test]
fn test_teddy_find_haystack_smaller_than_pattern() {
    let patterns: Vec<Vec<u8>> = vec![b"longpattern".to_vec()];
    let literals = Literals::new(&patterns);
    let teddy = Teddy::new(&literals).unwrap();
    let result = teddy.find(b"short");
    assert_eq!(result, None);
}

