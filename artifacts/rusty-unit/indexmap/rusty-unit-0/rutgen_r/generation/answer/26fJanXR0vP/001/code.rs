// Answer 0

#[test]
fn test_is_empty_when_set_has_no_elements() {
    use std::collections::HashMap;

    struct Set {
        map: HashMap<i32, i32>,
    }

    impl Set {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let set = Set { map: HashMap::new() };
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_when_set_has_elements() {
    use std::collections::HashMap;

    struct Set {
        map: HashMap<i32, i32>,
    }

    impl Set {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut set = Set { map: HashMap::new() };
    set.map.insert(1, 1);
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_when_set_has_multiple_elements() {
    use std::collections::HashMap;

    struct Set {
        map: HashMap<i32, i32>,
    }

    impl Set {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut set = Set { map: HashMap::new() };
    set.map.insert(1, 1);
    set.map.insert(2, 2);
    set.map.insert(3, 3);
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    use std::collections::HashMap;

    struct Set {
        map: HashMap<i32, i32>,
    }

    impl Set {
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
    }

    let mut set = Set { map: HashMap::new() };
    set.map.insert(1, 1);
    set.map.clear();
    assert!(set.is_empty());
}

