// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);

    // Test with existing key
    let result = {
        let entry = map.entry("poneyland");
        entry.or_insert(10)
    };
    
    assert_eq!(*result, 3);
    *result *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Test with nonexistent key
    let result = {
        let entry = map.entry("unicornland");
        entry.or_insert(5)
    };
    
    assert_eq!(*result, 5);
    assert_eq!(map["unicornland"], 5);
}

#[test]
#[should_panic]
fn test_or_insert_with_panic_on_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("testland", 1);

    // Correct use with existing key
    {
        let entry = map.entry("testland");
        assert_eq!(entry.or_insert(2), &mut 1);
    }

    // Intentionally causing a panic by trying a bad reference
    {
        let entry = map.entry("testland");
        let _panic_result = entry.or_insert(2);
        *entry.or_insert(3); // This will not panic, so we shouldn't reach here
    }
}

