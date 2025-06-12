// Answer 0

#[test]
fn test_insert_entry_creates_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(v) = map.entry("poneyland") {
        let o = v.insert_entry(37);
        assert_eq!(o.get(), &37);
    }
}

#[test]
fn test_insert_entry_overwrites_existing_value() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 37);

    if let Entry::Vacant(v) = map.entry("poneyland") {
        // This should not hit since the entry is occupied already.
        let _o = v.insert_entry(42);
        let occupied_entry = map.get("poneyland");
        assert_eq!(occupied_entry, Some(&37)); // Ensure the value remains 37
    }
    
    if let Entry::Occupied(o) = map.entry("poneyland") {
        let _replaced = o.insert(42); // This should replace the value of the occupied entry
        assert_eq!(o.get(), &42);
    }
}

#[test]
fn test_insert_entry_with_different_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("first", 1);

    if let Entry::Vacant(v) = map.entry("second") {
        let o = v.insert_entry(2);
        assert_eq!(o.get(), &2);
        assert_eq!(map.get("first"), Some(&1));
        assert_eq!(map.get("second"), Some(&2));
    }
}

#[test]
fn test_insert_entry_boundary_case() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(v) = map.entry("boundary") {
        // Insert the boundary value, which is zero
        let o = v.insert_entry(0);
        assert_eq!(o.get(), &0);
        assert_eq!(map.get("boundary"), Some(&0));
    }
}

