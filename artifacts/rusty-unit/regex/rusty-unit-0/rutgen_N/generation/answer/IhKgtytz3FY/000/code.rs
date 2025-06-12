// Answer 0

#[derive(Debug)]
struct Literals {
    complete: bool,
    longest_common_prefix: Vec<u8>,
    longest_common_suffix: Vec<u8>,
}

impl Literals {
    fn all_complete(&self) -> bool {
        self.complete
    }
    
    fn longest_common_prefix(&self) -> &Vec<u8> {
        &self.longest_common_prefix
    }
    
    fn longest_common_suffix(&self) -> &Vec<u8> {
        &self.longest_common_suffix
    }
}

#[derive(Debug)]
struct Matcher {
    pattern: String,
}

#[derive(Debug)]
struct FreqyPacked {
    data: Vec<u8>,
}

impl FreqyPacked {
    fn new(data: Vec<u8>) -> Self {
        FreqyPacked { data }
    }
}

#[derive(Debug)]
struct LiteralSearcher {
    complete: bool,
    lcp: FreqyPacked,
    lcs: FreqyPacked,
    matcher: Matcher,
}

impl LiteralSearcher {
    fn new(lits: Literals, matcher: Matcher) -> Self {
        let complete = lits.all_complete();
        LiteralSearcher {
            complete: complete,
            lcp: FreqyPacked::new(lits.longest_common_prefix().to_vec()),
            lcs: FreqyPacked::new(lits.longest_common_suffix().to_vec()),
            matcher: matcher,
        }
    }
}

#[test]
fn test_literal_searcher_new_complete() {
    let lits = Literals {
        complete: true,
        longest_common_prefix: vec![b'a', b'b', b'c'],
        longest_common_suffix: vec![b'x', b'y', b'z'],
    };
    let matcher = Matcher {
        pattern: String::from("abc"),
    };
    let searcher = LiteralSearcher::new(lits, matcher);
    assert!(searcher.complete);
    assert_eq!(searcher.lcp.data, vec![b'a', b'b', b'c']);
    assert_eq!(searcher.lcs.data, vec![b'x', b'y', b'z']);
}

#[test]
fn test_literal_searcher_new_incomplete() {
    let lits = Literals {
        complete: false,
        longest_common_prefix: vec![b'1', b'2', b'3'],
        longest_common_suffix: vec![],
    };
    let matcher = Matcher {
        pattern: String::from("xyz"),
    };
    let searcher = LiteralSearcher::new(lits, matcher);
    assert!(!searcher.complete);
    assert_eq!(searcher.lcp.data, vec![b'1', b'2', b'3']);
    assert_eq!(searcher.lcs.data, vec![]);
}

