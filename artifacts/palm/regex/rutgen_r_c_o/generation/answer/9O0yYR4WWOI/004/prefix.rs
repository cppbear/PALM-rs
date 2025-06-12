// Answer 0

#[test]
fn test_len_boyer_moore() {
    struct BoyerMooreSearch {
        pattern: Vec<u8>,
    }
    
    let search = BoyerMooreSearch {
        pattern: b"test".to_vec(),
    };
    
    let lits = Literals::empty(); // Initialize Literals
    let matcher = Matcher::BoyerMoore(search);
    
    let searcher = LiteralSearcher::new(lits, matcher);
    let result = searcher.len();
}

#[test]
fn test_len_empty_boyer_moore() {
    struct BoyerMooreSearch {
        pattern: Vec<u8>,
    }

    let search = BoyerMooreSearch {
        pattern: b"".to_vec(),
    };

    let lits = Literals::empty(); // Initialize Literals
    let matcher = Matcher::BoyerMoore(search);

    let searcher = LiteralSearcher::new(lits, matcher);
    let result = searcher.len();
}

