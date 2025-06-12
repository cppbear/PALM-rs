// Answer 0

#[test]
fn test_matcher_new_freqy_packed() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl TestLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let test_literals = TestLiterals {
        literals: vec![b"abcde".to_vec(), b"fghij".to_vec()],
    };

    let test_sset = SingleByteSet {
        sparse: vec![false; 26], // 26 single bytes (all ASCII)
        dense: vec![], // Density is less than 26
        complete: false,
        all_ascii: true,
    };

    let matcher = Matcher::new(&test_literals, test_sset);

    if let Matcher::FreqyPacked(freqy_packed) = matcher {
        assert_eq!(freqy_packed.pat, b"fghij".to_vec());
    } else {
        panic!("Expected Matcher::FreqyPacked but got a different matcher type.");
    }
}

#[test]
fn test_matcher_new_aho_corasick() {
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl TestLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let test_literals = TestLiterals {
        literals: vec![b"xyz".to_vec(), b"abc".to_vec()],
    };

    let test_sset = SingleByteSet {
        sparse: vec![false; 26], // 26 single bytes (all ASCII)
        dense: vec![],           // Density is less than 26
        complete: false,
        all_ascii: true,
    };

    let matcher = Matcher::new(&test_literals, test_sset);

    if let Matcher::AC(_) = matcher {
        // This condition is satisfied for Aho-Corasick matcher, asserting the type is correct.
    } else {
        panic!("Expected Matcher::AC but got a different matcher type.");
    }
}

