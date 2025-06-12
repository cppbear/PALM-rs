// Answer 0

#[test]
fn test_find_freqypacked_haystack_found() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let haystack = vec![b'x', b'y', b'a', b'b', b'c', b'd', b'z'];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.find(&haystack);
}

#[test]
fn test_find_freqypacked_haystack_not_found() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let haystack = vec![b'x', b'y', b'z'];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.find(&haystack);
}

#[test]
fn test_find_freqypacked_haystack_empty() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let haystack = vec![];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.find(&haystack);
}

#[test]
fn test_find_freqypacked_haystack_exact_match() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let haystack = pattern.clone();
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.find(&haystack);
}

#[test]
fn test_find_freqypacked_haystack_large_input() {
    let pattern = (0..255).map(|i| i as u8).collect::<Vec<u8>>(); // max pattern length
    let haystack = (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(); // valid haystack
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.find(&haystack);
}

