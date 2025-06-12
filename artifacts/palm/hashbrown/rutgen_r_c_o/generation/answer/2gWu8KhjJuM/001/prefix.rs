// Answer 0

#[test]
fn test_index_valid_key_a() {
    let map: HashMap<&str, &str> = [("a", "One"), ("b", "Two")].into();
    let _value = map[&"a"];
}

#[test]
fn test_index_valid_key_b() {
    let map: HashMap<&str, &str> = [("a", "One"), ("b", "Two")].into();
    let _value = map[&"b"];
}

#[should_panic]
fn test_index_invalid_key_c() {
    let map: HashMap<&str, &str> = [("a", "One"), ("b", "Two")].into();
    let _value = map[&"c"];
}

#[should_panic]
fn test_index_invalid_key_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let _value = map[&"a"];
}

#[test]
fn test_index_valid_key_multitype() {
    let map: HashMap<i32, &str> = [(1, "One"), (2, "Two")].into();
    let _value = map[&1];
    let _value = map[&2];
}

#[should_panic]
fn test_index_invalid_key_multitype() {
    let map: HashMap<i32, &str> = [(1, "One"), (2, "Two")].into();
    let _value = map[&3];
}

