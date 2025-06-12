// Answer 0

#[test]
fn test_retain_function_with_removal() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);
    
    map.retain(|&k, _| k % 2 == 0);
    
    assert_eq!(map.len(), 4);
    
    let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_unstable();
    assert_eq!(vec, [(0, 0), (2, 20), (4, 40), (6, 60)]);
}

#[test]
fn test_retain_function_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    
    map.retain(|&k, _| k % 2 == 0);
    
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_function_no_removal() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..4).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 4);
    
    map.retain(|&k, _| k % 2 != 0);
    
    assert_eq!(map.len(), 2);
    
    let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_unstable();
    assert_eq!(vec, [(1, 10), (3, 30)]);
}

#[test]
fn test_retain_function_all_removed() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..5).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 5);
    
    map.retain(|_, _| false);
    
    assert_eq!(map.len(), 0);
}

