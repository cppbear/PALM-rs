// Answer 0

#[test]
fn test_retain_removes_correct_elements() {
    use crate::HashMap;
    
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder::new(), Global);
    for x in 0..8 {
        map.insert(x, x * 10);
    }
    assert_eq!(map.len(), 8);

    map.retain(|&k, _| k % 2 == 0);

    assert_eq!(map.len(), 4);

    let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_unstable();
    assert_eq!(vec, [(0, 0), (2, 20), (4, 40), (6, 60)]);
}

#[test]
fn test_retain_keeps_elements_when_all_match() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), Global);
    for x in 0..4 {
        map.insert(x, x * 10);
    }
    assert_eq!(map.len(), 4);

    map.retain(|_, _| true);

    assert_eq!(map.len(), 4);
}

#[test]
fn test_retain_clears_all_elements() {
    use crate::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), Global);
    for x in 0..4 {
        map.insert(x, x * 10);
    }
    assert_eq!(map.len(), 4);
    
    map.retain(|_, _| false);

    assert_eq!(map.len(), 0);
}

