// Answer 0

#[test]
fn test_key_vacant_entry() {
    struct Vacant;
    
    impl Vacant {
        fn key(&self) -> &str {
            "x-hello"
        }
    }
    
    enum Entry {
        Vacant(Vacant),
    }

    let entry = Entry::Vacant(Vacant);
    
    match entry {
        Entry::Vacant(ref e) => {
            assert_eq!(e.key(), "x-hello");
        }
    }
}

#[test]
fn test_key_occupied_entry() {
    struct Occupied;
    
    impl Occupied {
        fn key(&self) -> &str {
            "y-world"
        }
    }
    
    enum Entry {
        Occupied(Occupied),
    }

    let entry = Entry::Occupied(Occupied);
    
    match entry {
        Entry::Occupied(ref e) => {
            assert_eq!(e.key(), "y-world");
        }
    }
}

