// Answer 0

#[test]
fn test_symmetric_difference_non_empty_sets() {
    let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    let set = &a ^ &b;

    let expected = [1, 2, 4, 5];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_first_set() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    let set = &a ^ &b;

    let expected = [3, 4, 5];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_second_set() {
    let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = HashSet::new();
    let set = &a ^ &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_empty_sets() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    let set = &a ^ &b;

    let expected: Vec<i32> = Vec::new();
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_identical_sets() {
    let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set = &a ^ &b;

    let expected: Vec<i32> = Vec::new();
    let mut i = 0;
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

