// Answer 0

#[test]
fn test_is_disjoint_empty_sets() {
    use indexmap::IndexSet;
    
    let set1: IndexSet<i32> = IndexSet::new();
    let set2: IndexSet<i32> = IndexSet::new();
    
    assert!(set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_non_overlapping_sets() {
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32> = IndexSet::new();
    let mut set2: IndexSet<i32> = IndexSet::new();

    set1.insert(1);
    set1.insert(2);
    set2.insert(3);
    set2.insert(4);

    assert!(set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_overlapping_sets() {
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32> = IndexSet::new();
    let mut set2: IndexSet<i32> = IndexSet::new();

    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);

    assert!(!set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_one_empty_set() {
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32> = IndexSet::new();
    let mut set2: IndexSet<i32> = IndexSet::new();

    set2.insert(1);
    set2.insert(2);

    assert!(set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_identical_sets() {
    use indexmap::IndexSet;

    let mut set1: IndexSet<i32> = IndexSet::new();
    let mut set2: IndexSet<i32> = IndexSet::new();

    set1.insert(1);
    set1.insert(2);
    set2.insert(1);
    set2.insert(2);

    assert!(!set1.is_disjoint(&set2));
}

