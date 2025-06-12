// Answer 0

#[test]
fn test_shift_insert_within_bounds() {
    use indexmap::IndexMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    let mut map: IndexMap<_, _, BuildHasherDefault<fnv::FnvHasher>> = IndexMap::default();
    map.insert("a", 1);
    map.insert("b", 2);

    let (key_ref, value_ref) = map.shift_insert(1, "c", 3);
    
    assert_eq!(*key_ref, "c");
    assert_eq!(*value_ref, 3);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
    assert_eq!(map.get("c"), Some(&3));
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds() {
    use indexmap::IndexMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    let mut map: IndexMap<_, _, BuildHasherDefault<fnv::FnvHasher>> = IndexMap::default();
    map.insert("a", 1);
    
    // This should panic since the index is out of bounds
    let _ = map.shift_insert(2, "b", 2);
}

