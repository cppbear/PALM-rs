// Answer 0

#[test]
fn test_get_disjoint_mut_unique_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let result = map.get_disjoint_mut([&2, &1]);
    assert_eq!(result, [Some(&mut 'c'), Some(&mut 'a')]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_duplicate_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&1, &1]);
}

#[test]
fn test_get_disjoint_mut_nonexistent_key() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let result = map.get_disjoint_mut([&4, &3]);
    assert_eq!(result, [None, Some(&mut 'b')]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_overlapping_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&1, &2, &1]);
}

