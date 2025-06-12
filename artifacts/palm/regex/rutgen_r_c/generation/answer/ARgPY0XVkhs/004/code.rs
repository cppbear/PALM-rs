// Answer 0

#[test]
fn test_new_boyer_moore() {
    #[derive(Clone, Debug)]
    struct TestLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl TestLiterals {
        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let test_lits = TestLiterals {
        literals: vec![b"test_pattern".to_vec(), b"another_pattern".to_vec()],
    };

    let test_sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z'],
        complete: false,
        all_ascii: true,
    };

    let matcher = Matcher::new(&test_lits, test_sset);
    match matcher {
        Matcher::BoyerMoore(boyer_moore) => {
            assert_eq!(boyer_moore.pattern, b"test_pattern".to_vec());
        },
        _ => panic!("Expected BoyerMoore variant"),
    }
}

