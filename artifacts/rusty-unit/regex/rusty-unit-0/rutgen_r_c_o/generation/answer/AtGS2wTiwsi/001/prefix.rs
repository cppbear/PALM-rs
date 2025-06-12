// Answer 0

#[test]
fn test_find_start_empty_haystack() {
    let searcher = LiteralSearcher::empty();
    let result = searcher.find_start(&[]);
}

#[test]
fn test_find_start_single_byte_haystack() {
    let lit = Literal::from_bytes(b"a");
    let lits = Literals::new(vec![lit]);
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_start(&[b'b']);
}

#[test]
fn test_find_start_haystack_is_shorter_than_literal() {
    let lit = Literal::from_bytes(b"abc");
    let lits = Literals::new(vec![lit]);
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_start(&[b'a']);
}

#[test]
fn test_find_start_haystack_with_length_two() {
    let lit = Literal::from_bytes(b"abc");
    let lits = Literals::new(vec![lit]);
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_start(&[b'a', b'b']);
}

