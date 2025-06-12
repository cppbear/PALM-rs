// Answer 0

#[test]
fn test_get_disjoint_mut_no_duplicates() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let result = map.get_disjoint_mut([&2, &1]);
    assert_eq!(result, [Some(&mut 'c'), Some(&mut 'a')]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_with_duplicates() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_mut([&1, &1]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_with_overlap() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
    let _ = map.get_disjoint_mut([&1, &2, &1]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_multiple_duplicates() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _ = map.get_disjoint_mut([&1, &1, &2]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_disjoint_mut_index_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a')]);
    let _ = map.get_disjoint_mut([&1, &3]);
}

