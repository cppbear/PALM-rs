// Answer 0

#[test]
fn test_get_disjoint_mut_overlapping_indices() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    
    // This should panic due to overlapping keys (keys 1 and 2 exist in the map)
    let result = std::panic::catch_unwind(|| {
        map.get_disjoint_mut([&1, &1]);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_index_out_of_bounds() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    
    // This should panic due to key being out of bounds (key 4 doesn’t exist in the map)
    let result = std::panic::catch_unwind(|| {
        map.get_disjoint_mut([&1, &4]);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_valid_keys() {
    let mut map = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
    
    // This should return valid mutable references without panic
    let results = map.get_disjoint_mut([&1, &2]);
    assert_eq!(results, [Some(&mut 'a'), Some(&mut 'b')]);
    
    // Verify mutation
    if let Some(val1) = results[0] {
        *val1 = 'x';
    }
    if let Some(val2) = results[1] {
        *val2 = 'y';
    }
    
    assert_eq!(map[1], 'x');
    assert_eq!(map[2], 'y');
}

