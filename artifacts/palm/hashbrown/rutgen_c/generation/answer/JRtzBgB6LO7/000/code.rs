// Answer 0

#[test]
fn test_extract_if_even_keys() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for i in 0..8 {
        map.insert(i, i);
    }

    let drained: Vec<(i32, i32)> = map.extract_if(|k, _v| k % 2 == 0).collect();

    let mut evens: Vec<i32> = drained.iter().map(|(k, _)| *k).collect();
    let mut odds: Vec<i32> = map.keys().cloned().collect();
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_non_exhaustion() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for i in 0..8 {
        map.insert(i, i);
    }

    {
        let _d = map.extract_if(|k, _v| k % 2 != 0);
    }

    assert_eq!(map.len(), 8);
}

#[test]
fn test_extract_if_empty_map() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder, Global);

    let drained: Vec<(i32, i32)> = map.extract_if(|_k, _v| true).collect();
    
    assert!(drained.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_extract_if_all_elements() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder, Global);
    for i in 0..5 {
        map.insert(i, i);
    }

    let drained: Vec<(i32, i32)> = map.extract_if(|_k, _v| true).collect();
    
    assert_eq!(drained.len(), 5);
    assert_eq!(map.len(), 0);
}

