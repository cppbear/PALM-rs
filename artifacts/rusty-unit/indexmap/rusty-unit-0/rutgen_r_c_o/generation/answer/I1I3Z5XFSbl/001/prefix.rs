// Answer 0

#[test]
fn test_binary_search_min_value() {
    let mut set = IndexSet::new();
    set.insert(0);
    let result = set.binary_search(&0);
}

#[test]
fn test_binary_search_max_value() {
    let mut set = IndexSet::new();
    set.insert(1000);
    let result = set.binary_search(&1000);
}

#[test]
fn test_binary_search_middle_value() {
    let mut set = IndexSet::new();
    set.insert(500);
    let result = set.binary_search(&500);
}

#[test]
fn test_binary_search_not_in_set_lower() {
    let mut set = IndexSet::new();
    set.insert(0);
    set.insert(500);
    set.insert(1000);
    let result = set.binary_search(&-1);
}

#[test]
fn test_binary_search_not_in_set_upper() {
    let mut set = IndexSet::new();
    set.insert(0);
    set.insert(500);
    set.insert(1000);
    let result = set.binary_search(&1001);
}

#[test]
fn test_binary_search_with_duplicates() {
    let mut set = IndexSet::new();
    set.insert(500);
    set.insert(500);  // Adding a duplicate
    set.insert(1000);
    let result = set.binary_search(&500);
}

#[test]
fn test_binary_search_multiple_elements() {
    let mut set = IndexSet::new();
    set.insert(100);
    set.insert(200);
    set.insert(300);
    let result = set.binary_search(&200);
}

#[test]
fn test_binary_search_empty_set() {
    let set = IndexSet::new();
    let result = set.binary_search(&500);
}

