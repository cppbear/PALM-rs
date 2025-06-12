// Answer 0

#[test]
fn test_truncate_shortens_set() {
    use indexmap::IndexMap;

    struct Set {
        map: IndexMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn len(&self) -> usize {
            self.map.len()
        }

        fn truncate(&mut self, len: usize) {
            self.map.truncate(len);
        }
    }

    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert_eq!(set.len(), 3);

    set.truncate(2);
    assert_eq!(set.len(), 2);

    set.truncate(1);
    assert_eq!(set.len(), 1);

    set.truncate(5);
    assert_eq!(set.len(), 1);
}

#[test]
fn test_truncate_empty_set() {
    use indexmap::IndexMap;

    struct Set {
        map: IndexMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: IndexMap::new(),
            }
        }

        fn truncate(&mut self, len: usize) {
            self.map.truncate(len);
        }

        fn len(&self) -> usize {
            self.map.len()
        }
    }

    let mut set = Set::new();
    assert_eq!(set.len(), 0);

    set.truncate(0);
    assert_eq!(set.len(), 0);
    
    set.truncate(5);
    assert_eq!(set.len(), 0);
}

