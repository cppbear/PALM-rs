// Answer 0

#[test]
fn test_get_occupied_entry() {
    use hashbrown::HashSet;

    struct MyEntry<'a> {
        key: &'a str,
    }

    impl<'a> MyEntry<'a> {
        fn get(self) -> &'a str {
            self.key
        }
    }

    struct Occupied<'a> {
        entry: MyEntry<'a>,
    }

    struct Vacant<'a> {
        key: &'a str,
    }

    enum Entry<'a> {
        Occupied(Occupied<'a>),
        Vacant(Vacant<'a>),
    }

    impl<'a> Entry<'a> {
        fn get(&self) -> &str {
            match self {
                Entry::Occupied(entry) => entry.entry.get(),
                Entry::Vacant(entry) => entry.key,
            }
        }
    }

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    let occupied_entry = Entry::Occupied(Occupied { entry: MyEntry { key: "poneyland" } });
    assert_eq!(occupied_entry.get(), "poneyland");
}

#[test]
fn test_get_vacant_entry() {
    use hashbrown::HashSet;

    struct MyEntry<'a> {
        key: &'a str,
    }

    impl<'a> MyEntry<'a> {
        fn get(self) -> &'a str {
            self.key
        }
    }

    struct Occupied<'a> {
        entry: MyEntry<'a>,
    }

    struct Vacant<'a> {
        key: &'a str,
    }

    enum Entry<'a> {
        Occupied(Occupied<'a>),
        Vacant(Vacant<'a>),
    }

    impl<'a> Entry<'a> {
        fn get(&self) -> &str {
            match self {
                Entry::Occupied(entry) => entry.entry.get(),
                Entry::Vacant(entry) => entry.key,
            }
        }
    }

    let mut set: HashSet<&str> = HashSet::new();
    
    let vacant_entry = Entry::Vacant(Vacant { key: "horseland" });
    set.insert("horseland");
    
    assert_eq!(vacant_entry.get(), "horseland");
}

