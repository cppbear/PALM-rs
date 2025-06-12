// Answer 0

#[test]
fn test_get_disjoint_mut_non_overlapping() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let result = map.get_disjoint_mut([&1, &2]);
    assert_eq!(result, [Some(&mut 'a'), Some(&mut 'b')]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_overlapping() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_mut([&1, &1]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_all_keys_overlapping() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_mut([&2, &2]);
}

#[test]
#[should_panic(expected = "dupKeys found")]
fn test_get_disjoint_mut_non_existing_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_mut([&3, &4]); // These keys do not exist
}

#[test]
fn test_get_disjoint_mut_index_out_of_bounds() {
    let mut map: indexmap::IndexMap<i32, char> = indexmap::IndexMap::new();
    assert!(map.get_disjoint_mut([&1; 5]).is_err());  // Panic due to out of bounds, after initialization
}

