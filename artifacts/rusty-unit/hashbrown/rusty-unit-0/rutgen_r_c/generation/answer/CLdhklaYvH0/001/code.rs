// Answer 0

#[test]
fn test_is_disjoint_empty_sets() {
    let set_a: HashSet<i32> = HashSet::new();
    let set_b: HashSet<i32> = HashSet::new();
    assert_eq!(set_a.is_disjoint(&set_b), true);
}

#[test]
fn test_is_disjoint_non_empty_and_empty() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    let set_b: HashSet<i32> = HashSet::new();
    assert_eq!(set_a.is_disjoint(&set_b), true);
}

#[test]
fn test_is_disjoint_same_elements() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(1);
    assert_eq!(set_a.is_disjoint(&set_b), false);
}

#[test]
fn test_is_disjoint_different_elements() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(2);
    assert_eq!(set_a.is_disjoint(&set_b), true);
}

#[test]
fn test_is_disjoint_multiple_elements() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(3);
    set_b.insert(4);
    assert_eq!(set_a.is_disjoint(&set_b), true);
}

#[test]
fn test_is_disjoint_shared_elements() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(2);
    set_b.insert(3);
    assert_eq!(set_a.is_disjoint(&set_b), false);
}

#[test]
fn test_is_disjoint_large_sets() {
    let mut set_a: HashSet<i32> = HashSet::new();
    for i in 0..1000 {
        set_a.insert(i);
    }
    let mut set_b: HashSet<i32> = HashSet::new();
    for i in 1000..2000 {
        set_b.insert(i);
    }
    assert_eq!(set_a.is_disjoint(&set_b), true);
}

