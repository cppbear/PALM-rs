// Answer 0

#[test]
fn test_is_disjoint_with_no_elements_in_common() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_empty_and_non_empty() {
    let a: HashSet<i32> = HashSet::new();
    let mut b = HashSet::new();
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
}

#[test]
fn test_is_disjoint_with_common_element() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);
}

#[test]
fn test_is_disjoint_with_multiple_common_elements() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(2);
    assert_eq!(a.is_disjoint(&b), false);
}

#[test]
fn test_is_disjoint_with_different_elements() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(3);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
}

