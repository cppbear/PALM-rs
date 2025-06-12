// Answer 0

#[test]
fn test_union_suffixes_non_empty_and_no_empty() {
    struct TestHir;

    impl TestHir {
        fn new() -> Self {
            TestHir
        }
    }

    struct TestSet {
        suffixes: Vec<String>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { suffixes: Vec::new() }
        }

        fn to_empty(&mut self) -> &mut Vec<String> {
            self.suffixes.clear();
            &mut self.suffixes
        }

        fn contains_empty(&self) -> bool {
            self.suffixes.contains(&"".to_string())
        }

        fn union(&mut self, lits: Vec<String>) -> bool {
            if self.suffixes.len() + lits.len() > 10 {
                return false;
            }
            self.suffixes.extend(lits);
            true
        }
        
        fn union_suffixes(&mut self, expr: &TestHir) -> bool {
            let mut lits = self.to_empty();
            // Simulating suffix extraction from expr
            lits.push("suffix1".to_string());
            lits.push("suffix2".to_string());
            lits.reverse();
            !lits.is_empty() && !self.contains_empty() && self.union(lits)
        }
    }

    let mut set = TestSet::new();
    let expr = TestHir::new();
    assert_eq!(set.union_suffixes(&expr), true);
}

#[test]
fn test_union_suffixes_with_empty_suffix() {
    struct TestHir;

    impl TestHir {
        fn new() -> Self {
            TestHir
        }
    }

    struct TestSet {
        suffixes: Vec<String>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { suffixes: Vec::new() }
        }

        fn to_empty(&mut self) -> &mut Vec<String> {
            self.suffixes.clear();
            &mut self.suffixes
        }

        fn contains_empty(&self) -> bool {
            self.suffixes.contains(&"".to_string())
        }

        fn union(&mut self, lits: Vec<String>) -> bool {
            if self.suffixes.len() + lits.len() > 10 {
                return false;
            }
            self.suffixes.extend(lits);
            true
        }
        
        fn union_suffixes(&mut self, expr: &TestHir) -> bool {
            let mut lits = self.to_empty();
            // Simulating suffix extraction from expr
            lits.push("".to_string()); // Adding empty suffix to test the condition
            lits.push("suffix2".to_string());
            lits.reverse();
            !lits.is_empty() && !self.contains_empty() && self.union(lits)
        }
    }

    let mut set = TestSet::new();
    let expr = TestHir::new();
    assert_eq!(set.union_suffixes(&expr), false);
}

