// Answer 0

#[test]
fn test_key_valid_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("apple").or_insert(10);
    match map.entry("apple") {
        Entry::Occupied(entry) => {
            let _ = entry.key();
        },
        _ => panic!(),
    }
}

#[test]
fn test_key_another_valid_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("banana").or_insert(20);
    match map.entry("banana") {
        Entry::Occupied(entry) => {
            let _ = entry.key();
        },
        _ => panic!(),
    }
}

#[test]
fn test_key_with_empty_string_and_panic() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("citrus").or_insert(5);
    match map.entry("") {
        Entry::Occupied(_) => panic!(),
        _ => {}
    }
}

#[test]
fn test_key_panic_on_uninserted_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    match map.entry("dragonfruit") {
        Entry::Occupied(_) => panic!(),
        _ => {}
    }
}

#[test]
fn test_key_special_character_input_and_panic() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("eclair").or_insert(15);
    match map.entry("eclair!") {
        Entry::Occupied(_) => panic!(),
        _ => {}
    }
}

