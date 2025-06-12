// Answer 0

#[test]
fn test_is_superset_empty_sets() {
    let set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    let set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.is_superset(&set_b);
}

#[test]
fn test_is_superset_single_element() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(1);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(1);
    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_not_superset() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(3);
    assert!(!set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_with_duplicates() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(1);
    set_a.insert(1); 
    set_a.insert(2);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(1);
    set_b.insert(2);
    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_all_elements() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(0);
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(0);
    set_b.insert(1);
    set_b.insert(2);
    assert!(set_a.is_superset(&set_b));
}

#[test]
fn test_is_superset_large_sets() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    for i in 0..100 {
        set_a.insert(i);
    }
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    for i in 50..100 {
        set_b.insert(i);
    }
    assert!(set_a.is_superset(&set_b));
}

