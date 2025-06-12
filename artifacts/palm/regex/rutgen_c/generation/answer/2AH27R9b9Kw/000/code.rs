// Answer 0

#[test]
fn test_find_no_matches() {
    let pats: Vec<Vec<u8>> = vec![b"abc".to_vec(), b"def".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).unwrap();

    let result = teddy.find(b"xyz");
    assert_eq!(result, None);
}

#[test]
fn test_find_with_exact_match() {
    let pats: Vec<Vec<u8>> = vec![b"abc".to_vec(), b"def".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).unwrap();

    let result = teddy.find(b"xyzabc123");
    assert_eq!(result, Some(Match { pat: 0, start: 3, end: 6 }));
}

#[test]
fn test_find_with_multiple_matches() {
    let pats: Vec<Vec<u8>> = vec![b"abc".to_vec(), b"def".to_vec(), b"cde".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).unwrap();

    let result = teddy.find(b"xyzabc123def456");
    assert_eq!(result, Some(Match { pat: 0, start: 3, end: 6 }));
}

#[test]
fn test_find_with_no_match_edge_case() {
    let pats: Vec<Vec<u8>> = vec![b"abc".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy = Teddy::new(&literals).unwrap();

    let result = teddy.find(b"ab");
    assert_eq!(result, None);
}

