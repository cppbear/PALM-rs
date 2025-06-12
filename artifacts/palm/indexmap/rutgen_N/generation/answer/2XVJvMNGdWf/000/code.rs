// Answer 0

#[derive(Clone, Debug)]
struct IndexSet<T, S> {
    elements: Vec<T>,
    // Other fields may exist
}

impl<T: PartialEq, S> IndexSet<T, S> {
    fn new() -> Self {
        IndexSet {
            elements: Vec::new(),
        }
    }

    fn insert(&mut self, value: T) {
        if !self.elements.contains(&value) {
            self.elements.push(value);
        }
    }

    fn symmetric_difference(&self, other: &IndexSet<T, S>) -> Vec<T> {
        let mut diff = Vec::new();
        
        for elem in &self.elements {
            if !other.elements.contains(elem) {
                diff.push(elem.clone());
            }
        }
        
        for elem in &other.elements {
            if !self.elements.contains(elem) {
                diff.push(elem.clone());
            }
        }
        
        diff
    }

    fn bitxor(self, other: &IndexSet<T, S>) -> Vec<T> {
        self.symmetric_difference(other).into_iter().collect()
    }
}

#[test]
fn test_bitxor_empty_sets() {
    let set1: IndexSet<i32, ()> = IndexSet::new();
    let set2: IndexSet<i32, ()> = IndexSet::new();
    
    let result = set1.clone().bitxor(&set2);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_bitxor_non_empty_sets() {
    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2 = IndexSet::new();
    set2.insert(2);
    set2.insert(3);
    
    let result = set1.clone().bitxor(&set2);
    assert_eq!(result, vec![1, 3]);
}

#[test]
fn test_bitxor_with_identical_sets() {
    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2 = IndexSet::new();
    set2.insert(1);
    set2.insert(2);
    
    let result = set1.clone().bitxor(&set2);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_bitxor_with_disjoint_sets() {
    let mut set1 = IndexSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2 = IndexSet::new();
    set2.insert(3);
    set2.insert(4);
    
    let result = set1.clone().bitxor(&set2);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

