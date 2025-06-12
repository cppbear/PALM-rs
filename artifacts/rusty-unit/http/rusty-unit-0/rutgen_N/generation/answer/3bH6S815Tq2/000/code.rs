// Answer 0

#[test]
fn test_is_empty_when_extensions_are_empty() {
    struct Extensions {
        map: Option<std::collections::HashMap<usize, usize>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn is_empty(&self) -> bool {
            self.map.as_ref().map_or(true, |map| map.is_empty())
        }

        pub fn insert(&mut self, key: usize, value: usize) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            if let Some(map) = self.map.as_mut() {
                map.insert(key, value);
            }
        }
    }

    let ext = Extensions::new();
    assert!(ext.is_empty());
}

#[test]
fn test_is_empty_when_extensions_have_elements() {
    struct Extensions {
        map: Option<std::collections::HashMap<usize, usize>>,
    }

    impl Extensions {
        pub fn new() -> Self {
            Extensions { map: None }
        }

        pub fn is_empty(&self) -> bool {
            self.map.as_ref().map_or(true, |map| map.is_empty())
        }

        pub fn insert(&mut self, key: usize, value: usize) {
            if self.map.is_none() {
                self.map = Some(std::collections::HashMap::new());
            }
            if let Some(map) = self.map.as_mut() {
                map.insert(key, value);
            }
        }
    }

    let mut ext = Extensions::new();
    ext.insert(1, 10);
    assert!(!ext.is_empty());
}

