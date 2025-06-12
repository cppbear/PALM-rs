// Answer 0

#[cfg(test)]
mod tests {
    struct MyMap {
        map: std::collections::HashMap<String, String>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.map.len()
        }

        pub fn insert(&mut self, key: String, value: String) {
            self.map.insert(key, value);
        }
    }

    #[test]
    fn test_len_empty_map() {
        let my_map = MyMap::new();
        assert_eq!(my_map.len(), 0);
    }

    #[test]
    fn test_len_single_element() {
        let mut my_map = MyMap::new();
        my_map.insert("key1".to_string(), "value1".to_string());
        assert_eq!(my_map.len(), 1);
    }

    #[test]
    fn test_len_multiple_elements() {
        let mut my_map = MyMap::new();
        my_map.insert("key1".to_string(), "value1".to_string());
        my_map.insert("key2".to_string(), "value2".to_string());
        assert_eq!(my_map.len(), 2);
    }
    
    #[test]
    fn test_len_after_removal() {
        let mut my_map = MyMap::new();
        my_map.insert("key1".to_string(), "value1".to_string());
        my_map.insert("key2".to_string(), "value2".to_string());
        my_map.map.remove("key1");
        assert_eq!(my_map.len(), 1);
    }
}

