// Answer 0

#[test]
fn test_get_disjoint_indices_mut_valid_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let result = map.get_disjoint_indices_mut([2, 0]);
    assert_eq!(result, Ok([(&2, &mut 'c'), (&1, &mut 'a')]));
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_duplicate_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_indices_mut([0, 0]);
}

#[test]
#[should_panic]
fn test_get_disjoint_indices_mut_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_indices_mut([0, 5]);
}

#[test]
fn test_get_disjoint_indices_mut_empty_map() {
    let mut map = indexmap::IndexMap::new();
    let result = map.get_disjoint_indices_mut::<2>([0, 1]);
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_indices_mut_all_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let result = map.get_disjoint_indices_mut([0, 1, 2]);
    assert_eq!(result, Ok([(&1, &mut 'a'), (&2, &mut 'b'), (&3, &mut 'c')]));
}

