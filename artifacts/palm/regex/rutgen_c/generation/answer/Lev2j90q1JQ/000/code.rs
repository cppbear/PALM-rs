// Answer 0

#[test]
fn test_teddy_len_empty_patterns() {
    let pats: Vec<Vec<u8>> = vec![];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).expect("Failed to create Teddy");

    assert_eq!(teddy.len(), 0);
}

#[test]
fn test_teddy_len_non_empty_patterns() {
    let pats: Vec<Vec<u8>> = vec![b"pattern1".to_vec(), b"pattern2".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).expect("Failed to create Teddy");

    assert_eq!(teddy.len(), 2);
}

