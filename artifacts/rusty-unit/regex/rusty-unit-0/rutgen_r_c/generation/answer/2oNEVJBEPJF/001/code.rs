// Answer 0

#[test]
fn test_suffixes_empty_literals() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }
    
    impl MockLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals { literals: vec![] };
    let matcher = Matcher::suffixes(&lits);
    match matcher {
        Matcher::Empty => (),
        _ => panic!("Expected Matcher::Empty"),
    }
}

#[test]
fn test_suffixes_single_character_literal() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals { literals: vec![vec![b'a']] };
    let matcher = Matcher::suffixes(&lits);
    
    if let Matcher::Bytes(sset) = matcher {
        assert!(sset.complete);
        assert_eq!(sset.dense, vec![b'a']);
    } else {
        panic!("Expected Matcher::Bytes");
    }
}

#[test]
fn test_suffixes_multiple_literals() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals { literals: vec![vec![b'a'], vec![b'b'], vec![b'c']] };
    let matcher = Matcher::suffixes(&lits);
    
    if let Matcher::Bytes(sset) = matcher {
        assert!(!sset.complete);
        assert_eq!(sset.dense, vec![b'a', b'b', b'c']);
    } else {
        panic!("Expected Matcher::Bytes");
    }
}

#[test]
fn test_suffixes_different_case_literals() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals { literals: vec![vec![b'A'], vec![b'b'], vec![b'c']] };
    let matcher = Matcher::suffixes(&lits);
    
    if let Matcher::Bytes(sset) = matcher {
        assert!(!sset.all_ascii);
        assert_eq!(sset.dense, vec![b'A', b'b', b'c']);
    } else {
        panic!("Expected Matcher::Bytes");
    }
}

