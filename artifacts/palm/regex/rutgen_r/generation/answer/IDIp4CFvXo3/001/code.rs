// Answer 0

#[test]
fn test_any_complete_empty_set() {
    struct TestSet {
        lits: Vec<TestLit>,
    }
    
    struct TestLit {
        complete: bool,
    }
    
    impl TestLit {
        fn is_cut(&self) -> bool {
            !self.complete
        }
    }

    let set = TestSet { lits: vec![] };
    assert!(!set.any_complete());
}

#[test]
fn test_any_complete_all_incomplete() {
    struct TestSet {
        lits: Vec<TestLit>,
    }
    
    struct TestLit {
        complete: bool,
    }
    
    impl TestLit {
        fn is_cut(&self) -> bool {
            !self.complete
        }
    }

    let set = TestSet { lits: vec![TestLit { complete: false }, TestLit { complete: false }] };
    assert!(!set.any_complete());
}

#[test]
fn test_any_complete_some_complete() {
    struct TestSet {
        lits: Vec<TestLit>,
    }
    
    struct TestLit {
        complete: bool,
    }
    
    impl TestLit {
        fn is_cut(&self) -> bool {
            !self.complete
        }
    }

    let set = TestSet { lits: vec![TestLit { complete: false }, TestLit { complete: true }] };
    assert!(set.any_complete());
}

#[test]
fn test_any_complete_all_complete() {
    struct TestSet {
        lits: Vec<TestLit>,
    }
    
    struct TestLit {
        complete: bool,
    }
    
    impl TestLit {
        fn is_cut(&self) -> bool {
            !self.complete
        }
    }

    let set = TestSet { lits: vec![TestLit { complete: true }, TestLit { complete: true }] };
    assert!(set.any_complete());
}

