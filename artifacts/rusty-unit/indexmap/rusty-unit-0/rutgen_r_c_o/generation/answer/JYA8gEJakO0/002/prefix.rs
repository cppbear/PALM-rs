// Answer 0

#[test]
fn test_get_disjoint_mut_success() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    let _result = map.get_disjoint_mut([&1, &2]);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_panic_overlapping_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _result = map.get_disjoint_mut([&1, &1]);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_panic_index_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
    let _result = map.get_disjoint_mut([&11, &2]);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_panic_duplicate_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b')]);
    let _result = map.get_disjoint_mut([&1, &3, &1]);
}

#[test]
fn test_get_disjoint_mut_multiple_keys_success() {
    let mut map = indexmap::IndexMap::from([(4, 'd'), (5, 'e'), (6, 'f')]);
    let _result = map.get_disjoint_mut([&4, &5, &6]);
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_panic_more_than_capacity() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (5, 'e'), (6, 'f'), (7, 'g'), (8, 'h'), (9, 'i'), (10, 'j')]);
    let _result = map.get_disjoint_mut([&1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11]);
}

