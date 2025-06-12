// Answer 0

#[test]
fn test_is_subset_empty_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<u32> = HashSet::new();
    let set: HashSet<u32> = HashSet::new();

    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_equal_sets() {
    use hashbrown::HashSet;

    let sup: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
    let set: HashSet<u32> = [1, 2, 3].iter().cloned().collect();

    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_non_empty_proper_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<u32> = [1, 2, 3, 4].iter().cloned().collect();
    let set: HashSet<u32> = [1, 2, 3].iter().cloned().collect();

    assert_eq!(set.is_subset(&sup), true);
}

#[test]
fn test_is_subset_non_empty_not_subset() {
    use hashbrown::HashSet;

    let sup: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
    let set: HashSet<u32> = [1, 2, 4].iter().cloned().collect();

    assert_eq!(set.is_subset(&sup), false);
}

#[test]
fn test_is_subset_identical_elements_different_order() {
    use hashbrown::HashSet;

    let sup: HashSet<u32> = [3, 2, 1].iter().cloned().collect();
    let set: HashSet<u32> = [1, 2, 3].iter().cloned().collect();

    assert_eq!(set.is_subset(&sup), true);
}

