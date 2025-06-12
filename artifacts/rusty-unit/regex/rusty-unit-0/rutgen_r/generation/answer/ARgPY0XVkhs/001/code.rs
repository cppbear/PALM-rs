// Answer 0

#[test]
fn test_new_empty_literals() {
    struct Literals {
        literals: Vec<Vec<u8>>,
    }

    impl Literals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    struct SingleByteSet {
        dense: Vec<u8>,
        complete: bool,
        all_ascii: bool,
    }

    let lits = Literals { literals: vec![] };
    let sset = SingleByteSet { dense: vec![], complete: false, all_ascii: false };

    let result = new(&lits, sset);
    match result {
        Matcher::Empty => {}
        _ => panic!("Expected Matcher::Empty"),
    }
}

#[test]
fn test_new_large_dense_single_byte_set() {
    struct Literals {
        literals: Vec<Vec<u8>>,
    }

    impl Literals {
        fn literals(&self) -> &[Vec<u8>] {
            &self.literals
        }
    }

    struct SingleByteSet {
        dense: Vec<u8>,
        complete: bool,
        all_ascii: bool,
    }

    let lits = Literals { literals: vec![b"test".to_vec()] };
    let sset = SingleByteSet { dense: vec![1; 26], complete: false, all_ascii: false };

    let result = new(&lits, sset);
    match result {
        Matcher::Empty => {}
        _ => panic!("Expected Matcher::Empty"),
    }
}

