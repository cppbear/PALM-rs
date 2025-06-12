// Answer 0

#[test]
fn test_insert_entry_normal_case() {
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::Custom };
    let value = "valid-value".parse().unwrap();

    if let Entry::Vacant(v) = map.try_entry(key).unwrap() {
        let mut e = v.try_insert_entry(value).unwrap();
        e.insert("new-value".parse().unwrap());
    }
}

#[test]
fn test_insert_entry_edge_cases() {
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::Custom };
    let value_short = "val".parse().unwrap();
    let value_long = "this-is-a-really-long-value".parse().unwrap();

    if let Entry::Vacant(v) = map.try_entry(key).unwrap() {
        let mut e_short = v.try_insert_entry(value_short).unwrap();
        e_short.insert("short-value".parse().unwrap());
    }

    if let Entry::Vacant(v) = map.try_entry(key).unwrap() {
        let mut e_long = v.try_insert_entry(value_long).unwrap();
        e_long.insert("long-value".parse().unwrap());
    }
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_insert_entry_panic_overflow() {
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::Custom };

    for i in 0..=MAX_SIZE {
        let value = format!("value{}", i).parse().unwrap();
        if let Entry::Vacant(v) = map.try_entry(key.clone()).unwrap() {
            let _ = v.try_insert_entry(value);
        }
    }
}

#[test]
fn test_insert_entry_multiple_entries() {
    let mut map = HeaderMap::new();
    let keys: Vec<HeaderName> = (0..10)
        .map(|i| HeaderName { inner: Repr::Custom })
        .collect();
    
    for key in keys {
        let value = format!("value-{}", key.hash()).parse().unwrap();
        if let Entry::Vacant(v) = map.try_entry(key).unwrap() {
            let mut e = v.try_insert_entry(value).unwrap();
            e.insert("updated-value".parse().unwrap());
        }
    }
}

