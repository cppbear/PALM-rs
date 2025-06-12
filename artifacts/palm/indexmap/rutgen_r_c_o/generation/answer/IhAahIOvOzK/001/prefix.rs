// Answer 0

#[test]
fn test_last_non_empty_index_set() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.last();
}

#[test]
fn test_last_single_element_index_set() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.insert(42);
    index_set.last();
}

#[test]
fn test_last_empty_index_set() {
    let index_set: IndexSet<i32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.last();
}

#[test]
fn test_last_after_removal_of_last() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.pop();
    index_set.last();
}

