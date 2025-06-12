// Answer 0

#[test]
fn test_find_teddy_avx2_with_valid_haystack() {
    let pats: Vec<Vec<u8>> = vec![b"test".to_vec(), b"example".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy_avx2 = TeddyAVX2::new(&literals).unwrap();
    let searcher = LiteralSearcher::new(literals, Matcher::TeddyAVX2(teddy_avx2));
    let haystack: &[u8] = b"This is a test string with example in it.";
    let _ = searcher.find(haystack);
}

#[test]
fn test_find_teddy_avx2_with_edge_haystack() {
    let pats: Vec<Vec<u8>> = vec![b"pattern".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy_avx2 = TeddyAVX2::new(&literals).unwrap();
    let searcher = LiteralSearcher::new(literals, Matcher::TeddyAVX2(teddy_avx2));
    let haystack: &[u8] = b"";
    let _ = searcher.find(haystack);
}

#[test]
fn test_find_teddy_avx2_with_large_haystack() {
    let pats: Vec<Vec<u8>> = vec![b"largepattern".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy_avx2 = TeddyAVX2::new(&literals).unwrap();
    let searcher = LiteralSearcher::new(literals, Matcher::TeddyAVX2(teddy_avx2));
    let haystack: Vec<u8> = vec![b'a'; 512].into_iter().chain(b"largepattern".iter()).chain(vec![b'b'; 512]).collect();
    let _ = searcher.find(&haystack);
}

#[test]
fn test_find_teddy_avx2_matching_multiple_patterns() {
    let pats: Vec<Vec<u8>> = vec![b"first".to_vec(), b"second".to_vec(), b"third".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy_avx2 = TeddyAVX2::new(&literals).unwrap();
    let searcher = LiteralSearcher::new(literals, Matcher::TeddyAVX2(teddy_avx2));
    let haystack: &[u8] = b"this string has first and also has second.";
    let _ = searcher.find(haystack);
}

#[test]
fn test_find_teddy_avx2_with_one_character_patterns() {
    let pats: Vec<Vec<u8>> = vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
    let literals = Literals::from(pats.clone());
    let teddy_avx2 = TeddyAVX2::new(&literals).unwrap();
    let searcher = LiteralSearcher::new(literals, Matcher::TeddyAVX2(teddy_avx2));
    let haystack: &[u8] = b"abcdef";
    let _ = searcher.find(haystack);
}

