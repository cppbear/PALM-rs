// Answer 0

#[test]
fn test_is_superset_empty_set() {
    use hashbrown::HashSet;

    let empty: HashSet<_> = HashSet::new();
    let sub: HashSet<_> = HashSet::new();

    assert_eq!(empty.is_superset(&sub), true);
}

#[test]
fn test_is_superset_containing_all_elements() {
    use hashbrown::HashSet;

    let sub: HashSet<_> = [1, 2].into_iter().collect();
    let mut set = HashSet::new();

    set.insert(0);
    set.insert(1);
    set.insert(2);

    assert_eq!(set.is_superset(&sub), true);
}

#[test]
fn test_is_superset_not_containing_all_elements() {
    use hashbrown::HashSet;

    let sub: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();

    set.insert(0);
    set.insert(1);

    assert_eq!(set.is_superset(&sub), false);
}

#[test]
fn test_is_superset_identical_sets() {
    use hashbrown::HashSet;

    let set: HashSet<_> = [1, 2].into_iter().collect();

    assert_eq!(set.is_superset(&set), true);
}

#[test]
fn test_is_superset_with_nil_subset() {
    use hashbrown::HashSet;

    let nil_subset: HashSet<_> = HashSet::new();
    let mut set = HashSet::new();

    set.insert(5);

    assert_eq!(set.is_superset(&nil_subset), true);
}

#[test]
fn test_is_superset_with_disjoint_sets() {
    use hashbrown::HashSet;

    let sub: HashSet<_> = [4, 5, 6].into_iter().collect();
    let mut set = HashSet::new();
    
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.is_superset(&sub), false);
}

