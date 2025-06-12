// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    let mut set = indexmap::set::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert_eq!(set.swap_remove_index(1), Some("b"));
    assert_eq!(set.len(), 2);
    assert!(!set.contains("b"));
}

#[test]
fn test_swap_remove_index_first_element() {
    let mut set = indexmap::set::IndexSet::new();
    set.insert("x");
    set.insert("y");
    set.insert("z");

    assert_eq!(set.swap_remove_index(0), Some("x"));
    assert_eq!(set.len(), 2);
    assert!(!set.contains("x"));
}

#[test]
fn test_swap_remove_index_last_element() {
    let mut set = indexmap::set::IndexSet::new();
    set.insert("p");
    set.insert("q");

    assert_eq!(set.swap_remove_index(1), Some("q"));
    assert_eq!(set.len(), 1);
    assert!(!set.contains("q"));
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_high() {
    let mut set = indexmap::set::IndexSet::new();
    set.insert("1");
    set.insert("2");

    let _ = set.swap_remove_index(2); // This should panic
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_low() {
    let mut set = indexmap::set::IndexSet::new();
    set.insert("A");

    let _ = set.swap_remove_index(usize::MAX); // This should panic
}

