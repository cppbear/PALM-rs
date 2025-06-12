// Answer 0

#[test]
fn test_find_end_matching_literal() {
    let haystack: Vec<u8> = b"Hello World!".to_vec();
    let literal: Vec<u8> = b"World!".to_vec();
    
    let freqy_packed = FreqyPacked {
        pat: literal.clone(),
        char_len: literal.len(),
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let result = searcher.find_end(&haystack);
}

#[test]
fn test_find_end_complete_match() {
    let haystack: Vec<u8> = b"abcdefg".to_vec();
    let literal: Vec<u8> = b"efg".to_vec();
    
    let freqy_packed = FreqyPacked {
        pat: literal.clone(),
        char_len: literal.len(),
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let result = searcher.find_end(&haystack);
}

#[test]
fn test_find_end_edge_case() {
    let haystack: Vec<u8> = b"z".to_vec();
    let literal: Vec<u8> = b"z".to_vec();
    
    let freqy_packed = FreqyPacked {
        pat: literal.clone(),
        char_len: literal.len(),
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let result = searcher.find_end(&haystack);
}

#[test]
fn test_find_end_multiple_occurrences() {
    let haystack: Vec<u8> = b"abcdabcde".to_vec();
    let literal: Vec<u8> = b"cde".to_vec();
    
    let freqy_packed = FreqyPacked {
        pat: literal.clone(),
        char_len: literal.len(),
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let result = searcher.find_end(&haystack);
}

#[test]
fn test_find_end_long_haystack() {
    let haystack: Vec<u8> = (b'a'..=b'z').cycle().take(65536).collect();
    let literal: Vec<u8> = b"xyz".to_vec();
    
    let freqy_packed = FreqyPacked {
        pat: literal.clone(),
        char_len: literal.len(),
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let result = searcher.find_end(&haystack);
}

