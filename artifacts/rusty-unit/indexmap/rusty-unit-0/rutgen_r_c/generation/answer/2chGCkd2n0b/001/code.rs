// Answer 0

#[test]
fn test_get_disjoint_indices_mut_valid_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let result = map.get_disjoint_indices_mut([2, 0]);
    assert_eq!(result, Ok([(&2, &mut 'c'), (&1, &mut 'a')]));
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_indices_mut([3]); // index 3 is out of bounds
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_duplicate_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_indices_mut([0, 0]); // duplicate index
}

#[test]
fn test_get_disjoint_indices_mut_multiple_valid_indices() {
    let mut map = indexmap::IndexMap::from([(0, 'x'), (1, 'y'), (2, 'z'), (3, 'w')]);
    let result = map.get_disjoint_indices_mut([3, 1, 2]);
    assert_eq!(result, Ok([(&3, &mut 'w'), (&1, &mut 'y'), (&2, &mut 'z')]));
}

#[test]
fn test_get_disjoint_indices_mut_minimal_case() {
    let mut map = indexmap::IndexMap::from([(5, 'm')]);
    let result = map.get_disjoint_indices_mut([0]);
    assert_eq!(result, Ok([(&5, &mut 'm')]));
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_empty_map() {
    let mut map: indexmap::IndexMap<i32, char> = indexmap::IndexMap::new();
    let _ = map.get_disjoint_indices_mut([0]); // out of bounds in an empty map
}

