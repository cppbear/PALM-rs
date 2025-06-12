// Answer 0

#[test]
fn test_find_with_empty_haystack() {
    let searcher = LiteralSearcher::empty();
    let result = searcher.find(&[]);
}

#[test]
fn test_find_with_non_empty_haystack_and_no_match() {
    let literals = Literals::empty();
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"test".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"hello world");
}

#[test]
fn test_find_with_single_match() {
    let literals = Literals::from(vec![b"test".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"test".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"test");
}

#[test]
fn test_find_with_multiple_match() {
    let literals = Literals::from(vec![b"test".to_vec(), b"hello".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"test".to_vec(), b"hello".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"hello test");
}

#[test]
fn test_find_with_haystack_longer_than_match() {
    let literals = Literals::from(vec![b"test".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"test".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"this should result in a test");
}

#[test]
fn test_find_with_match_at_end() {
    let literals = Literals::from(vec![b"cat".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"cat".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"there's a black cat");
}

#[test]
fn test_find_with_exact_match_at_start() {
    let literals = Literals::from(vec![b"hello".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"hello".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let result = searcher.find(b"hello world");
}

#[test]
fn test_find_with_full_capacity_haystack() {
    let literals = Literals::from(vec![b"match".to_vec()]);
    let matcher = Matcher::AC(FullAcAutomaton::new(vec![b"match".to_vec()]));
    let searcher = LiteralSearcher::new(literals, matcher);
    let haystack = vec![b'a'; 4096]; 
    let result = searcher.find(&haystack);
}

