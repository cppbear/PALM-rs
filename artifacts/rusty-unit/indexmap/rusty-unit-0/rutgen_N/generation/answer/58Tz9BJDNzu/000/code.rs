// Answer 0

#[test]
fn test_len_empty_set() {
    struct Set {
        map: std::collections::HashMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }
    }

    let set = Set::new();
    assert_eq!(set.len(), 0);
}

#[test]
fn test_len_non_empty_set() {
    struct Set {
        map: std::collections::HashMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }
    }

    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_len_duplicates() {
    struct Set {
        map: std::collections::HashMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }
    }

    let mut set = Set::new();
    set.insert(1);
    set.insert(1);
    assert_eq!(set.len(), 1);
} 

#[test]
fn test_len_after_removal() {
    struct Set {
        map: std::collections::HashMap<i32, ()>,
    }

    impl Set {
        fn new() -> Self {
            Set {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        pub fn remove(&mut self, value: &i32) {
            self.map.remove(value);
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }
    }

    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    set.remove(&1);
    assert_eq!(set.len(), 1);
}

