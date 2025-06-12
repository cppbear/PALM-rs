// Answer 0

#[test]
fn test_is_empty_with_empty_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<i32, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn is_empty(&self) -> bool {
            self.map.as_ref().map_or(true, |map| map.is_empty())
        }

        pub fn insert(&mut self, key: i32) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            self.map.as_mut().unwrap().insert(key, 0);
        }
    }

    let ext = Extensions::new();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_extensions() {
    struct Extensions {
        map: Option<std::collections::HashMap<i32, i32>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn is_empty(&self) -> bool {
            self.map.as_ref().map_or(true, |map| map.is_empty())
        }

        pub fn insert(&mut self, key: i32) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            self.map.as_mut().unwrap().insert(key, 0);
        }
    }

    let mut ext = Extensions::new();
    ext.insert(5i32);
    assert!(!ext.is_empty());
}

