// Answer 0

#[test]
fn test_suffixes_with_empty_literals() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }
    
    impl TestLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }
    
    let lits = TestLiterals { literals: vec![] };
    let matcher = Matcher::suffixes(&lits);
    match matcher {
        Matcher::Empty => {}
        _ => panic!("Expected Matcher::Empty"),
    }
}

#[test]
fn test_suffixes_with_single_character_literal() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }
    
    impl TestLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }
    
    let lits = TestLiterals { literals: vec![b"a".to_vec()] };
    let matcher = Matcher::suffixes(&lits);
    match matcher {
        Matcher::Bytes(_) => {}
        _ => panic!("Expected Matcher::Bytes"),
    }
}

#[test]
fn test_suffixes_with_multiple_literals() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }
    
    impl TestLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }
    
    let lits = TestLiterals { literals: vec![b"test".to_vec(), b"sample".to_vec()] };
    let matcher = Matcher::suffixes(&lits);
    match matcher {
        Matcher::AC(_) => {}
        _ => panic!("Expected Matcher::AC"),
    }
}

#[test]
fn test_suffixes_with_all_ascii_literals() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }
    
    impl TestLiterals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    let lits = TestLiterals { literals: vec![b"hello".to_vec(), b"world".to_vec()] };
    let matcher = Matcher::suffixes(&lits);
    match matcher {
        Matcher::Bytes(_) => {}
        _ => panic!("Expected Matcher::Bytes"),
    }
}

