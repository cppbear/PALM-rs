// Answer 0

#[test]
fn test_new_returns_freqy_packed() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl TestLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    struct TestSingleByteSet {
        dense: Vec<u8>,
        complete: bool,
        all_ascii: bool,
    }

    let lits = TestLiterals {
        literals: vec![b"test".to_vec(), b"sample".to_vec()],
    };
    
    let sset = TestSingleByteSet {
        dense: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        complete: false,
        all_ascii: true,
    };

    let result = new(&lits, sset);

    match result {
        Matcher::FreqyPacked(_) => (),
        _ => panic!("Expected Matcher::FreqyPacked, but got a different variant."),
    }
}

