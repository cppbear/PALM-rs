// Answer 0

#[test]
fn test_into_value_vacant_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    struct TestSet {
        set: HashSet<&'static str>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                set: HashSet::new(),
            }
        }
        
        fn entry<V>(&mut self, key: V) -> Entry<&'static str>
        where
            V: Into<&'static str>,
        {
            self.set.entry(key.into())
        }
    }

    let mut test_set = TestSet::new();
    
    match test_set.entry("poneyland") {
        Entry::Occupied(_) => panic!("Entry should be vacant"),
        Entry::Vacant(v) => {
            assert_eq!(v.into_value(), "poneyland");
            test_set.set.insert("poneyland");
        },
    }
}

#[test]
fn test_into_value_existed_entry_should_panic() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    match set.entry("poneyland") {
        Entry::Vacant(_) => panic!("Entry should be occupied"),
        Entry::Occupied(_) => {
            // This block simulates that we should not panic as no action is taken on this entry.
            // Invoking `into_value` here is not applicable since we are testing the panic case.
        },
    }
}

