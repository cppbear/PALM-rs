// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    struct CustomKey;
    impl Borrow<str> for CustomKey {
        fn borrow(&self) -> &str {
            "custom_key"
        }
    }

    let mut table = hashbrown::HashMap::new();

    for i in 1..=100 {
        table.insert(CustomKey, i);
    }

    let occupied_entry = EntryRef::Occupied(
        OccupiedEntry {
            hash: 1234,
            elem: Bucket::new(),
            table: &mut table,
        }
    );

    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&occupied_entry, &mut formatter);
}

#[test]
fn test_fmt_occupied_entry_with_different_values() {
    struct AnotherCustomKey;
    impl Borrow<str> for AnotherCustomKey {
        fn borrow(&self) -> &str {
            "another_custom_key"
        }
    }

    let mut map: hashbrown::HashMap<AnotherCustomKey, i32> = hashbrown::HashMap::new();
    
    for i in 1..=100 {
        map.insert(AnotherCustomKey, i);
    }
    
    let occupied_entry = EntryRef::Occupied(
        OccupiedEntry {
            hash: 5678,
            elem: Bucket::new(),
            table: &mut map,
        }
    );

    let mut formatter = fmt::Formatter::new();
    let _ = fmt(&occupied_entry, &mut formatter);
}

