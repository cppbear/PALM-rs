// Answer 0

#[test]
fn test_get_disjoint_mut_overlapping_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&1, &1]);
}

#[test]
fn test_get_disjoint_mut_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&4, &5]);
}

#[test]
fn test_get_disjoint_mut_multiple_overlaps() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&1, &2, &2]);
}

#[test]
fn test_get_disjoint_mut_same_key_multiple_times() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _ = map.get_disjoint_mut([&3, &3]);
}

