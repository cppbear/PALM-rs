// Answer 0

#[derive(Debug)]
struct IndexSet<T, S> {
    elements: Vec<T>,
    _marker: std::marker::PhantomData<S>,
}

impl<T, S> IndexSet<T, S> {
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            _marker: std::marker::PhantomData,
        }
    }
    
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }
}

struct CombinedIter<'a, T, S1, S> {
    iter: std::slice::Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}

#[test]
fn test_new_with_empty_sets() {
    let set1: IndexSet<i32, ()> = IndexSet::new(vec![]);
    let set2: IndexSet<i32, ()> = IndexSet::new(vec![]);
    
    let combined = CombinedIter {
        iter: set1.iter(),
        other: &set2,
    };

    assert!(combined.iter.clone().count() == 0);
}

#[test]
fn test_new_with_non_empty_set() {
    let set1: IndexSet<i32, ()> = IndexSet::new(vec![1, 2, 3]);
    let set2: IndexSet<i32, ()> = IndexSet::new(vec![4, 5]);
    
    let combined = CombinedIter {
        iter: set1.iter(),
        other: &set2,
    };

    assert!(combined.iter.clone().count() == 3);
}

#[test]
fn test_new_with_one_empty_and_one_non_empty_set() {
    let set1: IndexSet<i32, ()> = IndexSet::new(vec![1, 2, 3]);
    let set2: IndexSet<i32, ()> = IndexSet::new(vec![]);
    
    let combined = CombinedIter {
        iter: set1.iter(),
        other: &set2,
    };

    assert!(combined.iter.clone().count() == 3);
}

#[test]
fn test_new_with_same_elements() {
    let set1: IndexSet<i32, ()> = IndexSet::new(vec![1, 2, 3]);
    let set2: IndexSet<i32, ()> = IndexSet::new(vec![1, 2, 3]);
    
    let combined = CombinedIter {
        iter: set1.iter(),
        other: &set2,
    };

    assert!(combined.iter.clone().count() == 3);
}

