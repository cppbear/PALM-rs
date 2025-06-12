// Answer 0

#[test]
fn test_teddy_len_empty_patterns() {
    struct DummyLiterals;
    impl Literals for DummyLiterals {
        fn len(&self) -> usize { 0 }
    }

    let literals = DummyLiterals;
    let teddy = Teddy::new(&literals);
    assert!(teddy.is_none());
}

#[test]
fn test_teddy_len_non_empty_patterns() {
    struct DummyLiterals {
        patterns: Vec<Vec<u8>>,
    }
    
    impl Literals for DummyLiterals {
        fn len(&self) -> usize { self.patterns.len() }
    }

    let literals = DummyLiterals { patterns: vec![vec![b'a'], vec![b'b']] };
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.len(), 2);
}

#[test]
fn test_teddy_len_zero_patterns() {
    struct DummyLiterals {
        patterns: Vec<Vec<u8>>,
    }
    
    impl Literals for DummyLiterals {
        fn len(&self) -> usize { self.patterns.len() }
    }

    let literals = DummyLiterals { patterns: Vec::new() };
    let teddy = Teddy::new(&literals).unwrap();
    assert_eq!(teddy.len(), 0);
}

