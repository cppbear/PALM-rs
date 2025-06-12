// Answer 0

#[test]
fn test_get_disjoint_indices_mut_valid() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    assert_eq!(map.get_disjoint_indices_mut([2, 0]), Ok([(&2, &mut 'c'), (&1, &mut 'a')]));
}

#[test]
fn test_get_disjoint_indices_mut_unique_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    assert_eq!(map.get_disjoint_indices_mut([0, 1]), Ok([(&1, &mut 'a'), (&2, &mut 'b')]));
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_invalid_index_too_high() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_indices_mut([0, 2]); // Index 2 is out of bounds
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_invalid_index_repeated() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_indices_mut([0, 0]); // Same index cannot be used multiple times
}

#[test]
fn test_get_disjoint_indices_mut_empty() {
    let mut map: indexmap::IndexMap<i32, char> = indexmap::IndexMap::new();
    let result: Result<[(&i32, &mut char); 0], _> = map.get_disjoint_indices_mut([]);
    assert_eq!(result, Ok([]));
}

