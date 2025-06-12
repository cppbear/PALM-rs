// Answer 0

#[test]
fn test_append_non_empty_maps() {
    let mut a = IndexMap::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::from([(3, "c"), (4, "d")]);
    a.append(&mut b);
}

#[test]
fn test_append_empty_map_to_non_empty() {
    let mut a = IndexMap::from([(1, "a")]);
    let mut b = IndexMap::new();
    a.append(&mut b);
}

#[test]
fn test_append_non_empty_to_empty_map() {
    let mut a = IndexMap::new();
    let mut b = IndexMap::from([(1, "a"), (2, "b")]);
    a.append(&mut b);
}

#[test]
fn test_append_with_overwriting_keys() {
    let mut a = IndexMap::from([(1, "old")]);
    let mut b = IndexMap::from([(1, "new"), (2, "b")]);
    a.append(&mut b);
}

#[test]
fn test_append_large_maps() {
    let mut a = IndexMap::from_iter((0..500_000).map(|i| (i, i.to_string())));
    let mut b = IndexMap::from_iter((500_000..1_000_000).map(|i| (i, i.to_string())));
    a.append(&mut b);
}

#[test]
fn test_append_initial_capacity() {
    let mut a = IndexMap::with_capacity(100);
    let mut b = IndexMap::with_capacity(200);
    for i in 0..50 {
        a.insert(i, "foo");
        b.insert(i + 50, "bar");
    }
    a.append(&mut b);
}

#[should_panic]
fn test_append_with_invalid_state() {
    let mut a: IndexMap<i32, &str> = IndexMap::new();
    let mut b: IndexMap<i32, &str> = IndexMap::new();
    a.append(&mut b);
}

