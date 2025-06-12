// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        index_set.insert(i);
    }
    let search_value = 500;
    let result = index_set.binary_search_by_key(&search_value, |x| *x);
}

#[test]
fn test_binary_search_by_key_not_found() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        index_set.insert(i);
    }
    let search_value = 1001;
    let result = index_set.binary_search_by_key(&search_value, |x| *x);
}

#[test]
fn test_binary_search_by_key_first_element() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        index_set.insert(i);
    }
    let search_value = 0;
    let result = index_set.binary_search_by_key(&search_value, |x| *x);
}

#[test]
fn test_binary_search_by_key_last_element() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        index_set.insert(i);
    }
    let search_value = 999;
    let result = index_set.binary_search_by_key(&search_value, |x| *x);
}

#[test]
fn test_binary_search_by_key_empty_set() {
    let index_set: IndexSet<i32, RandomState> = IndexSet::new();
    let search_value = 0;
    let result = index_set.binary_search_by_key(&search_value, |x| *x);
}

