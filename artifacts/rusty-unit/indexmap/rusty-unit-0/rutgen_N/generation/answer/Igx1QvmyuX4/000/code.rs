// Answer 0

#[test]
fn test_get_range_empty() {
    let map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    let result = map.get_range(0..1);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_single_element() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    let result = map.get_range(0..1);
    let expected = Some(&indexmap::Slice::from_slice(&[(1, 10)]));
    assert_eq!(result, expected);
}

#[test]
fn test_get_range_multiple_elements() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let result = map.get_range(0..2);
    let expected = Some(&indexmap::Slice::from_slice(&[(1, 10), (2, 20)]));
    assert_eq!(result, expected);
}

#[test]
fn test_get_range_out_of_bounds_start() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let result = map.get_range(2..4);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_out_of_bounds_end() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let result = map.get_range(0..3);
    let expected = Some(&indexmap::Slice::from_slice(&[(1, 10), (2, 20)]));
    assert_eq!(result, expected);
}

#[test]
fn test_get_range_full_range() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let result = map.get_range(0..2);
    let expected = Some(&indexmap::Slice::from_slice(&[(1, 10), (2, 20)]));
    assert_eq!(result, expected);
}

#[test]
fn test_get_range_with_overlap() {
    let mut map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let result = map.get_range(1..3);
    let expected = Some(&indexmap::Slice::from_slice(&[(2, 20), (3, 30)]));
    assert_eq!(result, expected);
}

