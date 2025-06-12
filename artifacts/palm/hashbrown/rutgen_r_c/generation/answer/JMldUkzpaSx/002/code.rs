// Answer 0

#[test]
fn test_entry_occupied() {
    use crate::HashMap;

    // Create a HashMap instance.
    let mut map: HashMap<char, usize> = HashMap::new();

    // Insert a value into the map.
    map.insert('s', 2);
    
    // Simulate the entry method by looking for an existing key.
    let entry = map.entry('s');
    
    // Check that we get an occupied entry.
    match entry {
        crate::Entry::Occupied(occupied) => {
            assert_eq!(occupied.elem.ptr.as_ref(), &('s', 2));
            assert_eq!(occupied.hash, crate::make_hash(&map.hash_builder, &'s'));
            assert_eq!(occupied.table, &mut map);
        },
        _ => panic!("Expected Entry::Occupied but got a different variant"),
    }
}

#[test]
fn test_entry_vacant() {
    use crate::HashMap;

    // Create a HashMap instance.
    let mut map: HashMap<char, usize> = HashMap::new();

    // Simulate the entry method for a key that does not exist.
    let entry = map.entry('t');
    
    // Check we get a vacant entry.
    match entry {
        crate::Entry::Vacant(vacant) => {
            assert_eq!(vacant.key, 't');
            assert_eq!(vacant.hash, crate::make_hash(&map.hash_builder, &vacant.key));
            assert_eq!(vacant.table, &mut map);
        },
        _ => panic!("Expected Entry::Vacant but got a different variant"),
    }
}

