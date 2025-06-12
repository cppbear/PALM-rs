// Answer 0

#[test]
fn test_key_present() {
    let mut map: HashMap<&str, u32> = [("key1", 100), ("key2", 200)].into();
    let entry = map.raw_entry_mut().from_key(&"key1");
    if let RawEntryMut::Occupied(o) = entry {
        let _ = o.key();
    }
}

#[test]
fn test_key_present_long_string() {
    let mut map: HashMap<&str, u32> = [("long_key_string", 500), ("another_key", 300)].into();
    let entry = map.raw_entry_mut().from_key(&"long_key_string");
    if let RawEntryMut::Occupied(o) = entry {
        let _ = o.key();
    }
}

#[test]
fn test_key_present_edge_max_length() {
    let max_length_key = "a".repeat(100).as_str();
    let mut map: HashMap<&str, u32> = [(max_length_key, 400)].into();
    let entry = map.raw_entry_mut().from_key(max_length_key);
    if let RawEntryMut::Occupied(o) = entry {
        let _ = o.key();
    }
}

#[test]
fn test_key_present_zero_value() {
    let mut map: HashMap<&str, u32> = [("key_zero", 0)].into();
    let entry = map.raw_entry_mut().from_key(&"key_zero");
    if let RawEntryMut::Occupied(o) = entry {
        let _ = o.key();
    }
}

#[test]
fn test_key_present_high_value() {
    let mut map: HashMap<&str, u32> = [("key_high_value", 1_000_000)].into();
    let entry = map.raw_entry_mut().from_key(&"key_high_value");
    if let RawEntryMut::Occupied(o) = entry {
        let _ = o.key();
    }
}

#[test]
#[should_panic]
fn test_key_not_present() {
    let mut map: HashMap<&str, u32> = [("existing_key", 250)].into();
    let entry = map.raw_entry_mut().from_key(&"non_existing_key");
    if let RawEntryMut::Vacant(_) = entry {
        panic!();
    }
}

