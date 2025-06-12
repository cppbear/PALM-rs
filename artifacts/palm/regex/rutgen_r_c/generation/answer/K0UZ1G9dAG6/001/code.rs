// Answer 0

#[test]
fn test_matcher_prefixes_empty_literals() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = DummyLiterals { literals: vec![] };
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Empty => assert!(true),
        _ => assert!(false, "Expected Matcher::Empty"),
    }
}

#[test]
fn test_matcher_prefixes_single_byte_literal() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = DummyLiterals { literals: vec![vec![b'a']] };
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Bytes(sset) => {
            assert_eq!(sset.dense.len(), 1);
            assert!(sset.complete);
            assert!(sset.all_ascii);
        },
        _ => assert!(false, "Expected Matcher::Bytes"),
    }
}

#[test]
fn test_matcher_prefixes_multiple_literals() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = DummyLiterals { literals: vec![vec![b'a'], vec![b'b'], vec![b'c']] };
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Bytes(sset) => {
            assert_eq!(sset.dense.len(), 3);
            assert!(sset.complete);
            assert!(sset.all_ascii);
        },
        _ => assert!(false, "Expected Matcher::Bytes"),
    }
}

#[test]
fn test_matcher_prefixes_non_ascii() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = DummyLiterals { literals: vec![vec![0xC2, 0xA9]] }; // © character
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Bytes(sset) => {
            assert_eq!(sset.dense.len(), 1);
            assert!(!sset.all_ascii);
        },
        _ => assert!(false, "Expected Matcher::Bytes"),
    }
}

#[test]
fn test_matcher_prefixes_large_single_byte_literal() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lit = vec![b'a'; 30]; // Large single byte literal
    let lits = DummyLiterals { literals: vec![lit] };
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Empty => assert!(true),
        _ => assert!(false, "Expected Matcher::Empty"),
    }
}

#[test]
fn test_matcher_prefixes_mixed_literals() {
    struct DummyLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl DummyLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = DummyLiterals { literals: vec![vec![b'a'], vec![b'b'], vec![0xC2, 0xA9]] }; // mix of ASCII and non-ASCII
    let matcher = Matcher::prefixes(&lits);
    match matcher {
        Matcher::Bytes(sset) => {
            assert_eq!(sset.dense.len(), 2); // Should only count ASCII
            assert!(!sset.all_ascii);
        },
        _ => assert!(false, "Expected Matcher::Bytes"),
    }
}

