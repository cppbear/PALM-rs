// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    use indexmap::{IndexMap, Slice};
    
    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let slice: Option<&mut Slice<&str, i32>> = map.get_range_mut(0..2);
    assert!(slice.is_some());
    let slice_mut = slice.unwrap();
    assert_eq!(slice_mut.as_ref()[0].0, &"a");
    assert_eq!(slice_mut.as_ref()[1].0, &"b");
}

#[test]
fn test_get_range_mut_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    
    let slice: Option<&mut Slice<&str, i32>> = map.get_range_mut(0..1);
    assert!(slice.is_none());
}

#[test]
fn test_get_range_mut_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);

    let slice: Option<&mut Slice<&str, i32>> = map.get_range_mut(1..3);
    assert!(slice.is_none());
}

#[test]
fn test_get_range_mut_full_range() {
    use indexmap::{IndexMap, Slice};

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    let slice: Option<&mut Slice<&str, i32>> = map.get_range_mut(0..2);
    assert!(slice.is_some());
    let slice_mut = slice.unwrap();
    assert_eq!(slice_mut.as_ref()[0].0, &"a");
    assert_eq!(slice_mut.as_ref()[1].0, &"b");
}

