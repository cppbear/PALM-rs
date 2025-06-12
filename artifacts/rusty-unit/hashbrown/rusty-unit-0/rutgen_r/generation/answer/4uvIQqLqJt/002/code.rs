// Answer 0

#[test]
fn test_entry_get_occupied() {
    use hashbrown::HashSet;

    struct Entry<T> {
        value: T,
    }

    impl<T> Entry<T> {
        fn get(&self) -> &T {
            &self.value
        }
    }

    enum EntryWrapper<T> {
        Occupied(Entry<T>),
        Vacant,
    }

    impl<T> EntryWrapper<T> {
        fn get(&self) -> &T {
            match self {
                EntryWrapper::Occupied(entry) => entry.get(),
                EntryWrapper::Vacant => panic!("Attempted to get from a vacant entry"),
            }
        }
    }

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    let occupied_entry = EntryWrapper::Occupied(Entry { value: "poneyland" });
    assert_eq!(occupied_entry.get(), &"poneyland");
}

#[test]
#[should_panic(expected = "Attempted to get from a vacant entry")]
fn test_entry_get_vacant() {
    use hashbrown::HashSet;

    struct Entry<T> {
        value: T,
    }

    impl<T> Entry<T> {
        fn get(&self) -> &T {
            &self.value
        }
    }

    enum EntryWrapper<T> {
        Occupied(Entry<T>),
        Vacant,
    }

    impl<T> EntryWrapper<T> {
        fn get(&self) -> &T {
            match self {
                EntryWrapper::Occupied(entry) => entry.get(),
                EntryWrapper::Vacant => panic!("Attempted to get from a vacant entry"),
            }
        }
    }

    let vacant_entry = EntryWrapper::Vacant;
    vacant_entry.get();
}

