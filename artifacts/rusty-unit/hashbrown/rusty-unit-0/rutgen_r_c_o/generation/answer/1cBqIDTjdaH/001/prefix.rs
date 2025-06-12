// Answer 0

#[test]
fn test_into_values_non_empty() {
    let mut map = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let values: IntoValues<_, _, _> = map.into_values();
}

#[test]
fn test_into_values_empty() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let values: IntoValues<_, _, _> = map.into_values();
}

#[test]
fn test_into_values_with_capacity() {
    let mut map = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    for i in 0..10 {
        map.insert(i, i * 2);
    }
    let values: IntoValues<_, _, _> = map.into_values();
}

#[test]
fn test_into_values_large_capacity() {
    let mut map = HashMap::with_capacity_and_hasher_in(1_000_000, DefaultHashBuilder::new(), Global);
    for i in 0..1_000_000 {
        map.insert(i, i as usize);
    }
    let values: IntoValues<_, _, _> = map.into_values();
}

#[test]
fn test_into_values_panic_on_empty_map() {
    let map: HashMap<&str, i32> = HashMap::new();
    let values: IntoValues<_, _, _> = map.into_values();
}

