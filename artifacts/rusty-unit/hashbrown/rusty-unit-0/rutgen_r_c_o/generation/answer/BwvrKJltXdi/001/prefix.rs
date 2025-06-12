// Answer 0

#[test]
fn test_replace_entry_with_occupied() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = match map.entry("poneyland") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|k, v| {
                assert_eq!(k, &"poneyland");
                assert_eq!(v, 42);
                Some(v + 1)  // Return a new value, which meets the expectation
            })
        }
        Entry::Vacant(_) => panic!(),
    };

    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43); // Check the updated value
        }
        Entry::Vacant(_) => panic!(),
    }

    assert_eq!(map["poneyland"], 43); // Verify the map has the updated value
}

