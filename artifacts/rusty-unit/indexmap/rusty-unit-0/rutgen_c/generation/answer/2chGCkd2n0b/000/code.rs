// Answer 0

#[test]
fn test_get_disjoint_indices_mut_valid() {
    struct TestKey(usize);
    struct TestValue(char);

    let mut map = IndexMap::from([
        (TestKey(1), TestValue('a')),
        (TestKey(3), TestValue('b')),
        (TestKey(2), TestValue('c')),
    ]);

    let result = map.get_disjoint_indices_mut([2, 0]);
    assert!(result.is_ok());

    let pairs = result.unwrap();
    assert_eq!(pairs[0].0, &TestKey(2));
    assert_eq!(pairs[0].1, &mut TestValue('c'));
    assert_eq!(pairs[1].0, &TestKey(1));
    assert_eq!(pairs[1].1, &mut TestValue('a'));
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_out_of_bounds() {
    struct TestKey(usize);
    struct TestValue(char);

    let mut map = IndexMap::from([
        (TestKey(1), TestValue('a')),
        (TestKey(3), TestValue('b')),
        (TestKey(2), TestValue('c')),
    ]);

    let _ = map.get_disjoint_indices_mut([0, 3]); // Index 3 is out-of-bounds
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_duplicate_indices() {
    struct TestKey(usize);
    struct TestValue(char);

    let mut map = IndexMap::from([
        (TestKey(1), TestValue('a')),
        (TestKey(3), TestValue('b')),
        (TestKey(2), TestValue('c')),
    ]);

    let _ = map.get_disjoint_indices_mut([0, 0]); // Duplicate indices
}

