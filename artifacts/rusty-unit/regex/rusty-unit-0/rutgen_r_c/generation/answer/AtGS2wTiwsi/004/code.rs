// Answer 0

#[test]
fn test_find_start_empty_iter() {
    struct DummyLiterals;

    impl DummyLiterals {
        fn empty() -> LiteralSearcher {
            LiteralSearcher::empty()
        }
    }
    
    let searcher = DummyLiterals::empty();
    let result = searcher.find_start(b"example");
    assert_eq!(result, None);
}

#[test]
fn test_find_start_no_match() {
    struct DummyLiterals {
        items: Vec<u8>,
    }

    impl DummyLiterals {
        fn with_literals(items: Vec<u8>) -> LiteralSearcher {
            let matcher = Matcher::Bytes(SingleByteSet::new(&items));
            LiteralSearcher::new(Literals::from(items.clone()), matcher)
        }
    }

    let searcher = DummyLiterals::with_literals(vec![b'x', b'y', b'z']);
    let result = searcher.find_start(b"example");
    assert_eq!(result, None);
} 

#[test]
fn test_find_start_empty_haystack() {
    struct DummyLiterals {
        items: Vec<u8>,
    }

    impl DummyLiterals {
        fn with_literals(items: Vec<u8>) -> LiteralSearcher {
            let matcher = Matcher::Bytes(SingleByteSet::new(&items));
            LiteralSearcher::new(Literals::from(items.clone()), matcher)
        }
    }

    let searcher = DummyLiterals::with_literals(vec![b'a', b'b', b'c']);
    let result = searcher.find_start(b"");
    assert_eq!(result, None);
}

