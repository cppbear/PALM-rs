// Answer 0

#[test]
fn test_get_disjoint_mut_valid_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let values = map.get_disjoint_mut([&1, &2]);
    assert_eq!(values, [Some(&mut 'a'), Some(&mut 'b')]);
}

#[test]
fn test_get_disjoint_mut_duplicate_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let result = std::panic::catch_unwind(|| {
        map.get_disjoint_mut([&1, &1]);
    });
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let result = std::panic::catch_unwind(|| {
        map.get_disjoint_mut([&3, &2]);
    });
    assert!(result.is_err());
}

