// Answer 0

#[test]
fn test_map_key() {
    let input: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
    ]);
    let output: HashMap<&str, u8> = map_key(input);
    assert_eq!(output.len(), 3);
}

#[test]
fn test_map_val() {
    let input: HashMap<u8, &'static str> = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
    ]);
    let output: HashMap<u8, &str> = map_val(input);
    assert_eq!(output.len(), 3);
}

#[test]
fn test_iter_key() {
    let input: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
    ]);
    let iter: Iter<&'static str, u8> = input.iter();
    let output: Iter<&str, u8> = iter_key(iter);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_iter_val() {
    let input: HashMap<u8, &'static str> = HashMap::from([
        (1, "one"),
        (2, "two"),
    ]);
    let iter: Iter<u8, &'static str> = input.iter();
    let output: Iter<u8, &str> = iter_val(iter);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_into_iter_key() {
    let input: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
    ]);
    let iter: IntoIter<&'static str, u8> = input.into_iter();
    let output: IntoIter<&str, u8> = into_iter_key(iter);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_into_iter_val() {
    let input: HashMap<u8, &'static str> = HashMap::from([
        (1, "one"),
        (2, "two"),
    ]);
    let iter: IntoIter<u8, &'static str> = input.into_iter();
    let output: IntoIter<u8, &str> = into_iter_val(iter);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_keys_key() {
    let input: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
    ]);
    let keys: Keys<&'static str, u8> = input.keys();
    let output: Keys<&str, u8> = keys_key(keys);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_keys_val() {
    let input: HashMap<u8, &'static str> = HashMap::from([
        (1, "one"),
        (2, "two"),
    ]);
    let keys: Keys<u8, &'static str> = input.keys();
    let output: Keys<u8, &str> = keys_val(keys);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_values_key() {
    let input: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
    ]);
    let values: Values<&'static str, u8> = input.values();
    let output: Values<&str, u8> = values_key(values);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_values_val() {
    let input: HashMap<u8, &'static str> = HashMap::from([
        (1, "one"),
        (2, "two"),
    ]);
    let values: Values<u8, &'static str> = input.values();
    let output: Values<u8, &str> = values_val(values);
    assert_eq!(output.len(), 2);
}

#[test]
fn test_drain() {
    let input: HashMap<&'static str, &'static str> = HashMap::from([
        ("key1", "value1"),
        ("key2", "value2"),
    ]);
    let drain: Drain<'static, &'static str, &'static str> = input.drain();
    let output: Drain<&str, &str> = drain(drain);
    assert_eq!(output.len(), 2);
}

