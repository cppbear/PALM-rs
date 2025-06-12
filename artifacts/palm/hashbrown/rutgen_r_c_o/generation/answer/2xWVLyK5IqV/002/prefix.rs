// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("occupied".to_owned(), 100);

    let entry_ref = map.entry_ref("occupied");
    let _result: &mut usize = entry_ref.or_insert_with_key(|key| key.len());
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    let entry_ref = map.entry_ref("vacant_key");
    let _result: &mut usize = entry_ref.or_insert_with_key(|key| key.chars().count());
}

#[test]
fn test_or_insert_with_key_overwrite_existing_value() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("overwrite".to_owned(), 200);

    let entry_ref = map.entry_ref("overwrite");
    *entry_ref.or_insert_with_key(|key| key.len() * 2) *= 2;
}

#[test]
fn test_or_insert_with_key_multiple_empty_slots() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    let entry_ref1 = map.entry_ref("key1");
    let _result1: &mut usize = entry_ref1.or_insert_with_key(|key| key.len());

    let entry_ref2 = map.entry_ref("key2");
    let _result2: &mut usize = entry_ref2.or_insert_with_key(|key| key.chars().count() + 1);
}

#[test]
fn test_or_insert_with_key_with_special_character_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    let entry_ref = map.entry_ref("!@#$%^&*()");
    let _result: &mut usize = entry_ref.or_insert_with_key(|key| key.len() * 2);
}

