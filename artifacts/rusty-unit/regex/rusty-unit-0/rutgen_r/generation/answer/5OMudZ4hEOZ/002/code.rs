// Answer 0

#[test]
fn test_all_complete_non_empty_lits_all_complete() {
    struct Lit {
        cut: bool,
    }

    impl Lit {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    struct TestStruct {
        lits: Vec<Lit>,
    }

    impl TestStruct {
        fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let test_struct = TestStruct {
        lits: vec![Lit { cut: false }, Lit { cut: false }],
    };

    assert!(test_struct.all_complete());
}

#[test]
fn test_all_complete_non_empty_lits_some_incomplete() {
    struct Lit {
        cut: bool,
    }

    impl Lit {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    struct TestStruct {
        lits: Vec<Lit>,
    }

    impl TestStruct {
        fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let test_struct = TestStruct {
        lits: vec![Lit { cut: true }, Lit { cut: false }],
    };

    assert!(!test_struct.all_complete());
}

#[test]
fn test_all_complete_empty_lits() {
    struct Lit {
        cut: bool,
    }

    impl Lit {
        fn is_cut(&self) -> bool {
            self.cut
        }
    }

    struct TestStruct {
        lits: Vec<Lit>,
    }

    impl TestStruct {
        fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let test_struct = TestStruct {
        lits: vec![],
    };

    assert!(!test_struct.all_complete());
}

