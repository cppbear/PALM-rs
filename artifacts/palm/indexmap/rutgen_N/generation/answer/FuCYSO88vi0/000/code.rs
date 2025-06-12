// Answer 0

#[derive(Debug)]
struct IndexSet<T, S> {
    data: Vec<T>,
    state: S,
}

impl<T, S> IndexSet<T, S> {
    fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

struct SetIterator<'a, T, S, S1> {
    iter: std::slice::Iter<'a, T>,
    other: &'a IndexSet<T, S>,
}

#[test]
fn test_set_iterator_new() {
    let set1: IndexSet<i32, ()> = IndexSet {
        data: vec![1, 2, 3],
        state: (),
    };
    let set2: IndexSet<i32, ()> = IndexSet {
        data: vec![4, 5],
        state: (),
    };
    
    let iterator = SetIterator {
        iter: set1.iter(),
        other: &set2,
    };
    
    let mut iter_values: Vec<&i32> = iterator.iter.collect();
    assert_eq!(iter_values, vec![&1, &2, &3]);
}

#[test]
fn test_empty_set_iterator_new() {
    let set1: IndexSet<i32, ()> = IndexSet {
        data: vec![],
        state: (),
    };
    let set2: IndexSet<i32, ()> = IndexSet {
        data: vec![7],
        state: (),
    };
    
    let iterator = SetIterator {
        iter: set1.iter(),
        other: &set2,
    };
    
    let mut iter_values: Vec<&i32> = iterator.iter.collect();
    assert_eq!(iter_values, vec![]);
}

