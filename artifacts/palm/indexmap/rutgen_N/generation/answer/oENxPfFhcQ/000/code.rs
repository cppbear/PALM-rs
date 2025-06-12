// Answer 0

#[test]
fn test_new_function_with_inclusive_range() {
    use indexmap::IndexMap;
    use std::ops::RangeInclusive;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let range: RangeInclusive<usize> = 0..=1;
    let replace_with = vec![(4, 40), (5, 50)];

    let result = new(&mut map, range, replace_with.clone());

    assert_eq!(result.drain.len(), 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&3), Some(&30));
}

#[test]
fn test_new_function_with_exclusive_range() {
    use indexmap::IndexMap;
    use std::ops::Range;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let range: Range<usize> = 1..3;
    let replace_with = vec![(6, 60)];

    let result = new(&mut map, range, replace_with.clone());

    assert_eq!(result.drain.len(), 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&10));
}

#[should_panic]
#[test]
fn test_new_function_with_invalid_range() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    
    let range: std::ops::Range<usize> = 1..2; // Out of bounds
    
    let replace_with = vec![(4, 40)];
    
    let _result = new(&mut map, range, replace_with.clone());
}

