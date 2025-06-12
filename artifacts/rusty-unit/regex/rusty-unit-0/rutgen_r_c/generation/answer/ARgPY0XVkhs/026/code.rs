// Answer 0

#[test]
fn test_matcher_new_with_ac_automaton() {
    #[derive(Debug, Clone)]
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let literals = MockLiterals {
        literals: vec![b"test".to_vec()],
    };

    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: false,
    };

    let matcher = Matcher::new(&literals, single_byte_set);

    match matcher {
        Matcher::AC(_) => {},
        _ => panic!("Expected Matcher::AC"),
    }
}

