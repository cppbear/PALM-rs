// Answer 0

#[test]
fn test_sub_assign_less_rhs() {
    use crate::HashSet;
    use std::collections::hash_map::DefaultHasher;

    let mut a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [3].iter().cloned().collect();

    a -= &b;

    let expected = [1, 2];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_more_rhs() {
    use crate::HashSet;
    use std::collections::hash_map::DefaultHasher;

    let mut a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

    a -= &b;

    let expected = [1];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_empty_rhs() {
    use crate::HashSet;
    use std::collections::hash_map::DefaultHasher;

    let mut a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = HashSet::new(); // empty set

    a -= &b;

    let expected = [1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_sub_assign_equal_sets() {
    use crate::HashSet;
    use std::collections::hash_map::DefaultHasher;

    let mut a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

    a -= &b;

    assert!(a.is_empty());
}

