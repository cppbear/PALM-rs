// Answer 0

#[test]
fn test_len_with_empty_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<i32, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        pub fn insert(&mut self, value: i32) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            self.map.as_mut().unwrap().insert(value, value);
        }
    }

    let ext = Extensions::new();
    assert_eq!(ext.len(), 0);
}

#[test]
fn test_len_after_insert() {
    struct Extensions {
        map: Option<std::collections::HashMap<i32, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        pub fn insert(&mut self, value: i32) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            self.map.as_mut().unwrap().insert(value, value);
        }
    }

    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert_eq!(ext.len(), 1);
}

#[test]
fn test_len_after_multiple_inserts() {
    struct Extensions {
        map: Option<std::collections::HashMap<i32, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn len(&self) -> usize {
            self.map.as_ref().map_or(0, |map| map.len())
        }

        pub fn insert(&mut self, value: i32) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            self.map.as_mut().unwrap().insert(value, value);
        }
    }

    let mut ext = Extensions::new();
    ext.insert(1);
    ext.insert(2);
    ext.insert(3);
    assert_eq!(ext.len(), 3);
}

