// Answer 0

#[test]
fn test_key_occupied_entry() {
    struct MockOccupied {
        key: HeaderName,
    }

    impl MockOccupied {
        fn key(&self) -> &HeaderName {
            &self.key
        }
    }

    enum Entry {
        Vacant(MockVacant),
        Occupied(MockOccupied),
    }

    struct MockVacant {
        key: HeaderName,
    }

    impl MockVacant {
        fn key(&self) -> &HeaderName {
            &self.key
        }
    }

    struct HeaderName(String);

    let key_value = HeaderName(String::from("x-hello"));
    let occupied_entry = Entry::Occupied(MockOccupied { key: key_value });
    
    if let Entry::Occupied(ref e) = occupied_entry {
        assert_eq!(e.key(), &HeaderName(String::from("x-hello")));
    } else {
        panic!("Expected an occupied entry");
    }
}

