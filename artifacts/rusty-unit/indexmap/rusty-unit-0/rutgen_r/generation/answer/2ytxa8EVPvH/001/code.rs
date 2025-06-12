// Answer 0

#[derive(Debug, PartialEq, Eq, Clone)]
struct TestSet {
    elements: Vec<i32>,
}

impl TestSet {
    fn new(elements: Vec<i32>) -> Self {
        TestSet { elements }
    }

    fn union(&self, other: &TestSet) -> Vec<i32> {
        let mut result = self.elements.clone();
        for elem in &other.elements {
            if !result.contains(elem) {
                result.push(*elem);
            }
        }
        result
    }
}

#[test]
fn test_bitor_with_non_empty_sets() {
    let set_a = TestSet::new(vec![1, 2, 3]);
    let set_b = TestSet::new(vec![3, 4, 5]);
    
    let result = set_a.union(&set_b);
    
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_bitor_with_empty_second_set() {
    let set_a = TestSet::new(vec![1, 2, 3]);
    let set_b = TestSet::new(vec![]);
    
    let result = set_a.union(&set_b);
    
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_bitor_with_identical_sets() {
    let set_a = TestSet::new(vec![1, 2, 3]);
    let set_b = TestSet::new(vec![1, 2, 3]);
    
    let result = set_a.union(&set_b);
    
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_bitor_with_disjoint_sets() {
    let set_a = TestSet::new(vec![1, 3, 5]);
    let set_b = TestSet::new(vec![2, 4, 6]);
    
    let result = set_a.union(&set_b);
    
    assert_eq!(result, vec![1, 3, 5, 2, 4, 6]);
}

#[test]
fn test_bitor_with_empty_first_set() {
    let set_a = TestSet::new(vec![]);
    let set_b = TestSet::new(vec![1, 2, 3]);
    
    let result = set_a.union(&set_b);
    
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_bitor_with_large_sets() {
    let set_a = TestSet::new((0..1000).collect());
    let set_b = TestSet::new((500..1500).collect());
    
    let result = set_a.union(&set_b);
    
    let expected_result: Vec<i32> = (0..1500).collect();
    assert_eq!(result, expected_result);
}

