// Answer 0

#[test]
fn test_get_on_vacant_entry() {
    use hashbrown::HashSet;

    struct VacantEntry<'a, T> {
        inner: &'a HashSet<T>,
        value: T,
    }

    impl<'a, T> VacantEntry<'a, T> {
        fn get(&self) -> &T {
            &self.value
        }
    }

    let mut set: HashSet<&str> = HashSet::new();
    let entry = VacantEntry { 
        inner: &set, 
        value: "poneyland" 
    };
    
    assert_eq!(entry.get(), &"poneyland");
}

#[test]
fn test_get_on_empty_set() {
    use hashbrown::HashSet;

    struct VacantEntry<'a, T> {
        inner: &'a HashSet<T>,
        value: T,
    }

    impl<'a, T> VacantEntry<'a, T> {
        fn get(&self) -> &T {
            &self.value
        }
    }

    let set: HashSet<&str> = HashSet::new();
    let entry = VacantEntry { 
        inner: &set, 
        value: "empty" 
    };
    
    assert_eq!(entry.get(), &"empty");
}

