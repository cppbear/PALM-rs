// Answer 0

#[test]
fn test_symmetric_difference() {
    use crate::HashSet;

    let mut a: HashSet<i32> = HashSet::new();
    let mut b: HashSet<i32> = HashSet::new();

    a.insert(1);
    a.insert(2);
    a.insert(3);

    b.insert(4);
    b.insert(2);
    b.insert(3);
    b.insert(4);

    let sym_diff_a_b: HashSet<_> = a.symmetric_difference(&b).collect();
    let sym_diff_b_a: HashSet<_> = b.symmetric_difference(&a).collect();

    assert_eq!(sym_diff_a_b, sym_diff_b_a);
    assert_eq!(sym_diff_a_b, [1, 4].iter().copied().collect::<HashSet<_>>());
}

#[test]
fn test_symmetric_difference_with_empty_set() {
    use crate::HashSet;

    let mut a: HashSet<i32> = HashSet::new();
    let mut b: HashSet<i32> = HashSet::new();

    a.insert(1);
    a.insert(2);
    a.insert(3);

    let sym_diff_a_b: HashSet<_> = a.symmetric_difference(&b).collect();
    let sym_diff_b_a: HashSet<_> = b.symmetric_difference(&a).collect();

    assert_eq!(sym_diff_a_b, [1, 2, 3].iter().copied().collect::<HashSet<_>>());
    assert!(sym_diff_b_a.is_empty());
}

#[test]
fn test_symmetric_difference_no_common_elements() {
    use crate::HashSet;

    let mut a: HashSet<i32> = HashSet::new();
    let mut b: HashSet<i32> = HashSet::new();

    a.insert(1);
    a.insert(2);
    
    b.insert(3);
    b.insert(4);

    let sym_diff_a_b: HashSet<_> = a.symmetric_difference(&b).collect();
    assert_eq!(sym_diff_a_b, [1, 2, 3, 4].iter().copied().collect::<HashSet<_>>());
}

