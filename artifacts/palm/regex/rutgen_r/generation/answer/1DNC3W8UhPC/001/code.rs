// Answer 0

#[test]
fn test_is_empty_with_empty_lits() {
    struct Test {
        lits: Vec<Vec<String>>,
    }

    impl Test {
        pub fn is_empty(&self) -> bool {
            self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
        }
    }

    let test_instance = Test { lits: Vec::new() };
    assert!(test_instance.is_empty());
}

#[test]
fn test_is_empty_with_empty_member() {
    struct Test {
        lits: Vec<Vec<String>>,
    }

    impl Test {
        pub fn is_empty(&self) -> bool {
            self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
        }
    }

    let test_instance = Test { lits: vec![vec![]] };
    assert!(test_instance.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_member() {
    struct Test {
        lits: Vec<Vec<String>>,
    }

    impl Test {
        pub fn is_empty(&self) -> bool {
            self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
        }
    }

    let test_instance = Test { lits: vec![vec!["a".to_string()]] };
    assert!(!test_instance.is_empty());
}

#[test]
fn test_is_empty_with_multiple_empty_members() {
    struct Test {
        lits: Vec<Vec<String>>,
    }

    impl Test {
        pub fn is_empty(&self) -> bool {
            self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
        }
    }

    let test_instance = Test { lits: vec![vec![], vec![]] };
    assert!(test_instance.is_empty());
}

