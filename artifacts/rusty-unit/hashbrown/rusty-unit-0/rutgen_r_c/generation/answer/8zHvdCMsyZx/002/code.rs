// Answer 0

#[test]
fn test_is_subset_empty_set() {
    let empty_set: HashSet<i32> = HashSet::new();
    let sup_set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(empty_set.is_subset(&sup_set), true);
}

#[test]
fn test_is_subset_equal_sets() {
    let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    assert_eq!(set.is_subset(&set), true);
}

#[test]
fn test_is_subset_with_subset() {
    let sup_set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(2);
    assert_eq!(set.is_subset(&sup_set), true);
}

#[test]
fn test_is_subset_with_non_subset() {
    let sup_set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(4);
    assert_eq!(set.is_subset(&sup_set), false);
}

#[test]
fn test_is_subset_large_set() {
    let sup_set: HashSet<i32> = (1..=1000).collect();
    let mut set: HashSet<i32> = HashSet::new();
    for i in 1..=500 {
        set.insert(i);
    }
    assert_eq!(set.is_subset(&sup_set), true);
}

#[test]
fn test_is_subset_large_non_subset() {
    let sup_set: HashSet<i32> = (1..=1000).collect();
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1500);
    assert_eq!(set.is_subset(&sup_set), false);
}

