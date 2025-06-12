// Answer 0

#[test]
fn test_new_empty_literals() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    let matcher = Matcher::new(&lits, sset);
    matches!(matcher, Matcher::Empty);
}

#[test]
fn test_new_single_byte_set_complete() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![vec![b'a']]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![b'a'],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::new(&lits, sset);
    matches!(matcher, Matcher::Bytes(_));
}

#[test]
fn test_new_single_literal_with_boyer_moore() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j']]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j'],
        complete: false,
        all_ascii: true,
    };
    let matcher = Matcher::new(&lits, sset);
    matches!(matcher, Matcher::BoyerMoore(_));
}

#[test]
fn test_new_single_literal_with_freqy_packed() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![vec![b'a', b'b']]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: true,
    };
    let matcher = Matcher::new(&lits, sset);
    matches!(matcher, Matcher::FreqyPacked(_));
}

#[test]
fn test_new_teddy_avx2() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![vec![b"a"; 10], vec![b"b"; 10]]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: true,
    };
    
    unsafe { 
        if TeddyAVX2::available() {
            let matcher = Matcher::new(&lits, sset);
            matches!(matcher, Matcher::TeddyAVX2(_));
        }
    }
}

#[test]
fn test_new_teddy_ssse3() {
    struct MockLiterals {
        literals: Vec<Vec<u8>>,
    }

    impl MockLiterals {
        fn new(literals: Vec<Vec<u8>>) -> Self {
            MockLiterals { literals }
        }

        fn literals(&self) -> &Vec<Vec<u8>> {
            &self.literals
        }
    }

    let lits = MockLiterals::new(vec![vec![b"a"; 10], vec![b"b"; 10]]);
    let sset = SingleByteSet {
        sparse: vec![],
        dense: vec![b'a', b'b'],
        complete: false,
        all_ascii: true,
    };
    
    unsafe { 
        if TeddySSSE3::available() {
            let matcher = Matcher::new(&lits, sset);
            matches!(matcher, Matcher::TeddySSSE3(_));
        }
    }
}

