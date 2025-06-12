// Answer 0

#[test]
fn test_reserve_exact_no_additional() {
    let mut set = indexmap::IndexMap::<i32, ()>::new();
    set.reserve_exact(0);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_reserve_exact_single_additional() {
    let mut set = indexmap::IndexMap::<i32, ()>::new();
    set.reserve_exact(1);
    set.insert(1, ());
    assert_eq!(set.len(), 1);
}

#[test]
fn test_reserve_exact_multiple_additionals() {
    let mut set = indexmap::IndexMap::<i32, ()>::new();
    set.reserve_exact(5);
    for i in 0..5 {
        set.insert(i, ());
    }
    assert_eq!(set.len(), 5);
}

#[test]
fn test_reserve_exact_boundary_condition() {
    let mut set = indexmap::IndexMap::<i32, ()>::new();
    set.reserve_exact(100);
    for i in 0..100 {
        set.insert(i, ());
    }
    assert_eq!(set.len(), 100);
}

