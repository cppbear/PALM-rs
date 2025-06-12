// Answer 0

#[test]
fn test_find_empty_haystack() {
    let pats = Literals::new(vec![]);
    let teddy = Teddy::new(&pats).unwrap();
    let result = teddy.find(&[]);
}

#[test]
fn test_find_uninitialized_memory_haystack() {
    let pats = Literals::new(vec![]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack: &[u8] = unsafe { std::slice::from_raw_parts(std::ptr::null(), 1024) };
    let result = teddy.find(haystack);
}

#[test]
fn test_find_no_patterns() {
    let pats = Literals::new(vec![]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack = b"test haystack with some random data";
    let result = teddy.find(haystack);
}

#[test]
fn test_find_patterns_dont_exist() {
    let pats = Literals::new(vec![b"pattern1".to_vec(), b"pattern2".to_vec()]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack = b"this haystack does not contain the patterns";
    let result = teddy.find(haystack);
}

#[test]
fn test_find_large_haystack_no_patterns() {
    let pats = Literals::new(vec![]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack = vec![b'a'; 2048];
    let result = teddy.find(&haystack);
}

#[test]
fn test_find_patterns_length_zero() {
    let pats = Literals::new(vec![b"".to_vec()]);
    let teddy = Teddy::new(&pats).unwrap();
    let haystack = b"some other data";
    let result = teddy.find(haystack);
}

