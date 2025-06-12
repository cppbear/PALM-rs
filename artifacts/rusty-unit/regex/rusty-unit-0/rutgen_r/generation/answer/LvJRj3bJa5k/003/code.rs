// Answer 0

#[test]
fn test_trim_suffix_success() {
    struct Lit {
        data: Vec<u8>,
    }

    struct Literals {
        lits: Vec<Lit>,
    }

    impl Literals {
        fn min_len(&self) -> Option<usize> {
            self.lits.iter().map(|lit| lit.data.len()).min()
        }

        fn to_empty(&self) -> Literals {
            Literals { lits: Vec::new() }
        }
    }

    impl Lit {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn truncate(&mut self, new_len: usize) {
            self.data.truncate(new_len);
        }

        fn cut(&mut self) {
            // Assume cut modifies the data in some way; implementation is omitted for simplicity
        }
    }

    let literals = Literals {
        lits: vec![
            Lit { data: b"hello".to_vec() },
            Lit { data: b"world".to_vec() },
            Lit { data: b"rust".to_vec() },
        ],
    };

    let result = literals.trim_suffix(2);

    assert!(result.is_some());
    let new_literals = result.unwrap();
    assert_eq!(new_literals.lits.len(), 3);
    assert_eq!(new_literals.lits[0].data, b"hel");
    assert_eq!(new_literals.lits[1].data, b"wo");
    assert_eq!(new_literals.lits[2].data, b"r");
}

#[test]
#[should_panic]
fn test_trim_suffix_nothing_remaining() {
    struct Lit {
        data: Vec<u8>,
    }

    struct Literals {
        lits: Vec<Lit>,
    }

    impl Literals {
        fn min_len(&self) -> Option<usize> {
            self.lits.iter().map(|lit| lit.data.len()).min()
        }

        fn to_empty(&self) -> Literals {
            Literals { lits: Vec::new() }
        }
    }

    impl Lit {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn truncate(&mut self, new_len: usize) {
            self.data.truncate(new_len);
        }

        fn cut(&mut self) {
            // Assume cut modifies the data in some way; implementation is omitted for simplicity
        }
    }

    let literals = Literals {
        lits: vec![
            Lit { data: b"a".to_vec() },
            Lit { data: b"b".to_vec() },
        ],
    };

    let result = literals.trim_suffix(1);
    
    assert!(result.is_none());
}

