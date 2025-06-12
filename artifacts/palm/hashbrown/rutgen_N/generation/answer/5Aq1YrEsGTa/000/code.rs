// Answer 0

#[test]
fn test_retain_even_keys() {
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
fn test_retain_no_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|_, _| false);

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_all_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|_, _| true);

    assert_eq!(map.len(), 8);
}

#[test]
fn test_retain_single_element() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|&k, _| k == 3);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&3), Some(&30));
}

