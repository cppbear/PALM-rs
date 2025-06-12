// Answer 0

#[test]
fn test_retain_all_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|&k, _| true);
    assert_eq!(map.len(), 8);

    let vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    assert_eq!(vec, [(0, 0), (1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60), (7, 70)]);
}

#[test]
fn test_retain_no_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|&k, _| false);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_some_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 8);

    map.retain(|&k, _| k % 2 == 0);
    assert_eq!(map.len(), 4);

    let vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_unstable();
    assert_eq!(vec, [(0, 0), (2, 20), (4, 40), (6, 60)]);
}

#[test]
#[should_panic]
fn test_retain_panic_on_mutability() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..3).map(|x| (x, x * 10)).collect();
    assert_eq!(map.len(), 3);

    map.retain(|&k, v| {
        if k == 1 {
            *v += 5; // This should not panic, as `f` mutates `v` safely.
            false
        } else {
            true
        }
    });

    assert_eq!(map.len(), 2);
}

