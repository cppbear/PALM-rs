// Answer 0

#[test]
fn test_retain_with_all_true() {
    let mut map = IndexMap::new();
    for i in 0..100 {
        map.insert(i, i * 2);
    }
    map.retain(|_, _| true);
}

#[test]
fn test_retain_with_some_false() {
    let mut map = IndexMap::new();
    for i in 0..100 {
        map.insert(i, i * 2);
    }
    map.retain(|k, _| k % 2 == 0);
}

#[test]
fn test_retain_with_all_false() {
    let mut map = IndexMap::new();
    for i in 0..100 {
        map.insert(i, i * 2);
    }
    map.retain(|k, _| k > 100);
}

#[test]
fn test_retain_empty_map() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.retain(|_, _| true);
}

#[test]
fn test_retain_single_element_true() {
    let mut map = IndexMap::new();
    map.insert(1, 1);
    map.retain(|_, _| true);
}

#[test]
fn test_retain_single_element_false() {
    let mut map = IndexMap::new();
    map.insert(1, 1);
    map.retain(|_, _| false);
}

#[test]
fn test_retain_large_map() {
    let mut map = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, i + 1);
    }
    map.retain(|k, _| k < 500);
}

#[test]
fn test_retain_with_custom_condition() {
    let mut map = IndexMap::new();
    for i in 0..10 {
        map.insert(i, i * 3);
    }
    map.retain(|k, v| *v > 15);
}

