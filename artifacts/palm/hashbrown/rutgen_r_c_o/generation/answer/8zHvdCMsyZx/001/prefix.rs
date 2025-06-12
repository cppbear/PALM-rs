// Answer 0

#[test]
fn test_empty_subset() {
    let sup: HashSet<_> = HashSet::new();
    let set: HashSet<_> = HashSet::new();
    set.is_subset(&sup);
}

#[test]
fn test_equal_size_empty_subset() {
    let sup: HashSet<_> = HashSet::new();
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.is_subset(&sup);
}

#[test]
fn test_equal_size_subset() {
    let sup: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.is_subset(&sup);
}

#[test]
fn test_equal_size_not_subset() {
    let sup: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let mut set = HashSet::new();
    set.insert(4);
    set.insert(5);
    set.is_subset(&sup);
}

#[test]
fn test_large_equal_size_subset() {
    let sup: HashSet<_> = (0..1000).collect();
    let mut set = HashSet::new();
    for i in 0..500 {
        set.insert(i);
    }
    set.is_subset(&sup);
}

#[test]
fn test_large_equal_size_not_subset() {
    let sup: HashSet<_> = (0..1000).collect();
    let mut set = HashSet::new();
    for i in 500..1000 {
        set.insert(i);
    }
    set.is_subset(&sup);
}

#[test]
fn test_non_empty_empty_other() {
    let mut set = HashSet::new();
    set.insert(1);
    set.is_subset(&HashSet::new());
}

#[test]
fn test_identical_sets() {
    let sup: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let set = sup.clone();
    set.is_subset(&sup);
}

