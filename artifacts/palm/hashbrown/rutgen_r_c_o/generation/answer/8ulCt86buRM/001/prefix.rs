// Answer 0

#[test]
fn test_insert_unique_values() {
    let mut set = HashSet::new();
    for value in 0..=1000000 {
        set.insert(value);
    }
}

#[test]
fn test_insert_duplicate_values() {
    let mut set = HashSet::new();
    for value in 0..=1000000 {
        set.insert(value);
        assert_eq!(set.insert(value), false);
    }
}

#[test]
fn test_insert_negative_values() {
    let mut set = HashSet::new();
    let negatives = [-1, -2, -3, -1000000];
    for &value in &negatives {
        set.insert(value);
    }
}

#[test]
fn test_insert_edge_case_one_duplicate() {
    let mut set = HashSet::new();
    set.insert(0);
    assert_eq!(set.insert(0), false);
}

#[test]
fn test_insert_edge_case_maximum_value() {
    let mut set = HashSet::new();
    assert_eq!(set.insert(1000000), true);
    assert_eq!(set.insert(1000000), false);
}

#[test]
fn test_insert_edge_case_negative_and_positive() {
    let mut set = HashSet::new();
    assert_eq!(set.insert(-1), true);
    assert_eq!(set.insert(1000000), true);
    assert_eq!(set.insert(-1), false);
    assert_eq!(set.insert(1000000), false);
}

