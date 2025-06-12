// Answer 0

#[test]
fn test_retain_with_always_false() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    // Retain function that always returns false
    map.retain(|_, _| false);

    // All elements should be removed, so the length should be zero
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_always_true() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    // Retain function that always returns true
    map.retain(|_, _| true);

    // No elements should be removed, so the length should remain the same
    assert_eq!(map.len(), 8);
}

#[test]
fn test_retain_with_even_keys() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    // Retain only even keys
    map.retain(|&k, _| k % 2 == 0);

    // There should only be elements with keys 0, 2, 4, 6
    assert_eq!(map.len(), 4);
    let expected: Vec<(i32, i32)> = vec![(0, 0), (2, 20), (4, 40), (6, 60)];
    let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_unstable();
    assert_eq!(vec, expected);
}

#[test]
fn test_retain_on_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    assert_eq!(map.len(), 0);

    // Retain function that returns true
    map.retain(|_, _| true);

    // Should still be empty
    assert_eq!(map.len(), 0);
}

