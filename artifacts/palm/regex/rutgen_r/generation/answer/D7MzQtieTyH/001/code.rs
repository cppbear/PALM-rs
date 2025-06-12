// Answer 0

#[derive(Default)]
struct TestHir {
    suffixes: Vec<String>,
}

impl TestHir {
    fn new(suffixes: Vec<String>) -> Self {
        Self { suffixes }
    }
}

#[derive(Default)]
struct TestSet {
    literals: Vec<String>,
}

impl TestSet {
    fn to_empty(&self) -> Vec<String> {
        Vec::new()
    }

    fn contains_empty(&self) -> bool {
        self.literals.is_empty()
    }

    fn union(&mut self, lits: Vec<String>) -> bool {
        if self.literals.len() + lits.len() > 10 {
            return false;
        }
        self.literals.extend(lits);
        true
    }
}

impl TestSet {
    pub fn union_suffixes(&mut self, expr: &TestHir) -> bool {
        let mut lits = self.to_empty();
        lits.extend(expr.suffixes.iter().cloned());
        lits.reverse();
        !lits.is_empty() && !self.contains_empty() && self.union(lits)
    }
}

#[test]
fn test_union_suffixes_non_empty_and_valid() {
    let mut set = TestSet::default();
    let expr = TestHir::new(vec!["suffix1".to_string(), "suffix2".to_string()]);
    assert!(set.union_suffixes(&expr));
    assert_eq!(set.literals.len(), 2);
    assert_eq!(set.literals, vec!["suffix2", "suffix1"]);
}

#[test]
fn test_union_suffixes_empty_lits() {
    let mut set = TestSet::default();
    let expr = TestHir::new(vec!["".to_string()]); // will cause contains_empty to return true
    assert!(!set.union_suffixes(&expr));
    assert!(set.literals.is_empty());
}

#[test]
fn test_union_suffixes_exceed_length_limit() {
    let mut set = TestSet {
        literals: vec!["existing".to_string(); 10],
    };
    let expr = TestHir::new(vec!["new".to_string()]);
    assert!(!set.union_suffixes(&expr)); // union should fail due to size limit
    assert_eq!(set.literals.len(), 10);
}

#[test]
fn test_union_suffixes_valid_case_with_empty_strings() {
    let mut set = TestSet::default();
    let expr = TestHir::new(vec!["valid1".to_string(), "valid2".to_string()]);
    assert!(set.union_suffixes(&expr));
    assert_eq!(set.literals.len(), 2);
    assert_eq!(set.literals, vec!["valid2", "valid1"]);
}

