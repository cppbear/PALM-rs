// Answer 0

#[test]
fn test_binary_search_by_empty_set() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&5));
}

#[test]
fn test_binary_search_by_single_element_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.push(5);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&5));
}

#[test]
fn test_binary_search_by_single_element_not_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.push(5);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&3));
}

#[test]
fn test_binary_search_by_multiple_elements_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend(vec![1, 3, 5, 7, 9]);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&5));
}

#[test]
fn test_binary_search_by_multiple_elements_not_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend(vec![1, 3, 5, 7, 9]);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&6));
}

#[test]
fn test_binary_search_by_large_set_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend((0..1_000_000).step_by(2));
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&500_000));
}

#[test]
fn test_binary_search_by_large_set_not_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend((0..1_000_000).step_by(2));
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&500_001));
}

#[test]
fn test_binary_search_by_sorted_order() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend(vec![1, 3, 5, 7, 9]);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&1));
}

#[test]
#[should_panic]
fn test_binary_search_by_out_of_bounds() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.extend(vec![1, 3, 5, 7, 9]);
    index_set.sort();
    index_set.binary_search_by(|&x| x.cmp(&10));
}

