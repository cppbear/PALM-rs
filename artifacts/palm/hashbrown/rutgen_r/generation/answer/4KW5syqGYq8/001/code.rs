// Answer 0

#[test]
fn test_symmetric_difference_basic() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();
    
    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_no_common_elements() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![4, 5, 6].into_iter().collect();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 3, 4, 5, 6];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference_identical_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![1, 2, 3].into_iter().collect();

    let set = &a ^ &b;

    assert!(set.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_set() {
    use hashbrown::HashSet;

    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = HashSet::new();

    let set = &a ^ &b;

    let mut i = 0;
    let expected = [1, 2, 3];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
} 

#[test]
fn test_symmetric_difference_large_sets() {
    use hashbrown::HashSet;

    let a: HashSet<_> = (1..1001).collect(); // values from 1 to 1000
    let b: HashSet<_> = (500..1501).collect(); // values from 500 to 1500

    let set = &a ^ &b;

    let mut i = 0;
    let expected = (1..500).chain(1001..1501).collect::<Vec<_>>(); // expected values

    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

