// Answer 0

#[test]
fn test_find_end_same_length_no_match() {
    let haystack = b"abcdefg";
    let mut searcher = LiteralSearcher::empty();
    let literal = vec![b'x', b'y', b'z']; // no match
    searcher.matcher = Matcher::Bytes(SingleByteSet::from_literals(&literal));

    let result = searcher.find_end(haystack);
}

#[test]
fn test_find_end_empty_haystack() {
    let haystack: &[u8] = b"";
    let searcher = LiteralSearcher::empty();

    let result = searcher.find_end(haystack);
}

#[test]
fn test_find_end_non_matching_haystack() {
    let haystack = b"abcdefghij";
    let mut searcher = LiteralSearcher::empty();
    let literal = vec![b'k', b'l', b'm']; // no match
    searcher.matcher = Matcher::Bytes(SingleByteSet::from_literals(&literal));
    
    let result = searcher.find_end(haystack);
}

#[test] 
fn test_find_end_long_haystack_no_match() {
    let haystack = b"longhaystackwithoutmatches";
    let mut searcher = LiteralSearcher::empty();
    let literal = vec![b'n', b'o', b'p']; // no match
    searcher.matcher = Matcher::Bytes(SingleByteSet::from_literals(&literal));

    let result = searcher.find_end(haystack);
}

#[test] 
fn test_find_end_large_haystack_no_match() {
    let haystack = b"A".repeat(256).as_slice(); // 256 bytes
    let mut searcher = LiteralSearcher::empty();
    let literal = vec![b'B', b'C', b'D']; // no match
    searcher.matcher = Matcher::Bytes(SingleByteSet::from_literals(&literal));

    let result = searcher.find_end(haystack);
}

#[test]
fn test_find_end_literal_equal_to_haystack_no_match() {
    let haystack = b"literal";
    let mut searcher = LiteralSearcher::empty();
    let literal = haystack; // matching length, but actual literals don't match
    searcher.matcher = Matcher::Bytes(SingleByteSet::from_literals(&literal));

    let result = searcher.find_end(haystack);
}

