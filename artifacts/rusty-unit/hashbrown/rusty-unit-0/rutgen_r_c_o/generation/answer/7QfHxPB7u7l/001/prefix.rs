// Answer 0

#[test]
fn test_key_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    let entry_ref = map.entry_ref("horseland");
    let key = entry_ref.key();
}

#[test]
fn test_key_with_new_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    let entry_ref = map.entry_ref("unicornland");
    let key = entry_ref.key();
}

