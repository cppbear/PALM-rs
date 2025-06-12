// Answer 0

#[test]
fn test_find_empty_haystack() {
    let pats = Literals::new(&[]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = &[];
    teddy.find(haystack);
}

#[test]
fn test_find_small_haystack() {
    let pats = Literals::new(&[vec![1, 2]]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = &[0, 0, 0, 0];
    teddy.find(haystack);
}

#[test]
fn test_find_haystack_exact_size() {
    let pats = Literals::new(&[vec![1, 2]]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    teddy.find(haystack);
}

#[test]
fn test_find_large_haystack() {
    let pats = Literals::new(&[vec![2, 3], vec![4, 5]]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: Vec<u8> = (0..256).map(|x| x as u8).collect();
    teddy.find(&haystack);
}

#[test]
fn test_find_haystack_with_patterns_not_included() {
    let pats = Literals::new(&[vec![9, 10], vec![11, 12]]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    teddy.find(haystack);
}

#[test]
fn test_find_pats_not_malleable() {
    let pats = Literals::new(&[vec![0], vec![1]]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = &[7, 8, 9];
    teddy.find(haystack);
}

