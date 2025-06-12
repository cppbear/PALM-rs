// Answer 0

#[test]
fn test_is_subset_empty_set() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set: HashSet<_> = HashSet::new();

    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_elements_in_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    set.insert(2);

    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_with_elements_not_in_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set: HashSet<_> = HashSet::new();
    set.insert(2);
    set.insert(4);

    assert_eq!(set.is_subset(&sup), false);
}

#[test]
fn test_is_subset_equal_sets() {
    use hashbrown::HashSet;

    let mut set1: HashSet<_> = [1, 2, 3].into_iter().collect();
    let mut set2 = set1.clone();

    assert_eq!(set1.is_subset(&set2), true);
}

#[test]
fn test_is_subset_with_superset() {
    use hashbrown::HashSet;

    let mut sup: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();
    let mut set: HashSet<_> = [2, 4].into_iter().collect();

    assert_eq!(set.is_subset(&sup), true);
}

