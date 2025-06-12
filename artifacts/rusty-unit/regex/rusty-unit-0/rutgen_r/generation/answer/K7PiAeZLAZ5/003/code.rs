// Answer 0

#[test]
fn test_alternate_literals_successful_union() {
    struct TestHir;
    struct TestLiterals {
        limit_size: usize,
        empty: bool,
    }

    impl TestLiterals {
        fn to_empty(&mut self) -> TestLiterals {
            TestLiterals { limit_size: self.limit_size, empty: true }
        }
        
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }

        fn is_empty(&self) -> bool {
            self.empty
        }

        fn union(&mut self, other: &TestLiterals) -> bool {
            !other.is_empty()
        }

        fn cut(&mut self) {}

        fn cross_product(&self, other: &TestLiterals) -> bool {
            false // Controlled to return false for testing
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }
    }

    let mut lits = TestLiterals { limit_size: 10, empty: false };

    let es = vec![TestHir, TestHir];
    alternate_literals(&es, &mut lits, |_, lits3| {
        let mut new_lits = lits3.to_empty();
        new_lits.empty = false; // Set it to not empty
        new_lits
    });

    assert!(!lits.is_empty());
}

#[test]
#[should_panic]
fn test_alternate_literals_empty_lits3() {
    struct TestHir;
    struct TestLiterals {
        limit_size: usize,
        empty: bool,
    }

    impl TestLiterals {
        fn to_empty(&mut self) -> TestLiterals {
            TestLiterals { limit_size: self.limit_size, empty: true }
        }
        
        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }

        fn is_empty(&self) -> bool {
            self.empty
        }

        fn union(&mut self, other: &TestLiterals) -> bool {
            !other.is_empty() // Controlled to return false for empty literals
        }

        fn cut(&mut self) {}

        fn cross_product(&self, other: &TestLiterals) -> bool {
            false // Controlled to return false for testing
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }
    }

    let mut lits = TestLiterals { limit_size: 10, empty: false };

    let es = vec![TestHir, TestHir];
    alternate_literals(&es, &mut lits, |_, lits3| {
        let mut new_lits = lits3.to_empty();
        new_lits.empty = true; // Setting it to empty to trigger panic
        new_lits
    });
}

#[test]
#[should_panic]
fn test_alternate_literals_unsupported_union() {
    struct TestHir;
    struct TestLiterals {
        limit_size: usize,
        empty: bool,
    }

    impl TestLiterals {
        fn to_empty(&mut self) -> TestLiterals {
            TestLiterals { limit_size: self.limit_size, empty: true }
        }

        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }

        fn is_empty(&self) -> bool {
            self.empty
        }

        fn union(&mut self, other: &TestLiterals) -> bool {
            false // Controlled to return false for union
        }

        fn cut(&mut self) {}

        fn cross_product(&self, other: &TestLiterals) -> bool {
            false // Controlled to return false for testing
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }
    }

    let mut lits = TestLiterals { limit_size: 10, empty: false };

    let es = vec![TestHir, TestHir];
    alternate_literals(&es, &mut lits, |_, lits3| {
        let mut new_lits = lits3.to_empty();
        new_lits.empty = false; // Not empty but union will fail
        new_lits
    });
}

