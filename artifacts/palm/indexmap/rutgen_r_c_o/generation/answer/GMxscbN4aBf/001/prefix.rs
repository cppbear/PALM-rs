// Answer 0

#[test]
fn test_swap_remove_present() {
    let mut set = IndexSet::<i32, RandomState>::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.swap_remove(&2);
}

#[test]
fn test_swap_remove_not_present() {
    let mut set = IndexSet::<i32, RandomState>::new();
    set.insert(1);
    set.insert(3);
    let result = set.swap_remove(&2);
}

#[test]
fn test_swap_remove_last_element() {
    let mut set = IndexSet::<i32, RandomState>::new();
    set.insert(1);
    set.insert(2);
    set.swap_remove(&2);
}

#[test]
fn test_swap_remove_multiple_occurrences() {
    let mut set = IndexSet::<i32, RandomState>::new();
    set.insert(1);
    set.insert(1);
    set.insert(2);
    let result = set.swap_remove(&1);
}

#[test]
fn test_swap_remove_empty_set() {
    let mut set = IndexSet::<i32, RandomState>::new();
    let result = set.swap_remove(&1);
}

#[test]
fn test_swap_remove_edge_case() {
    let mut set = IndexSet::<i32, RandomState>::new();
    set.insert(0);
    let result = set.swap_remove(&0);
}

