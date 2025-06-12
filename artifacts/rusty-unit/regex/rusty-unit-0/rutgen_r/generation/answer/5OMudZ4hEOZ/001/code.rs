// Answer 0

#[test]
fn test_all_complete_empty() {
    struct Lit;

    impl Lit {
        fn is_cut(&self) -> bool {
            false
        }
    }

    struct CompleteSet {
        lits: Vec<Lit>,
    }

    impl CompleteSet {
        pub fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let complete_set = CompleteSet { lits: vec![] };
    assert!(!complete_set.all_complete());
}

#[test]
fn test_all_complete_non_empty_with_cuts() {
    struct Lit;

    impl Lit {
        fn is_cut(&self) -> bool {
            true
        }
    }

    struct CompleteSet {
        lits: Vec<Lit>,
    }

    impl CompleteSet {
        pub fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let complete_set = CompleteSet { lits: vec![Lit] };
    assert!(!complete_set.all_complete());
}

#[test]
fn test_all_complete_non_empty_without_cuts() {
    struct Lit;

    impl Lit {
        fn is_cut(&self) -> bool {
            false
        }
    }

    struct CompleteSet {
        lits: Vec<Lit>,
    }

    impl CompleteSet {
        pub fn all_complete(&self) -> bool {
            !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
        }
    }

    let complete_set = CompleteSet { lits: vec![Lit] };
    assert!(complete_set.all_complete());
}

