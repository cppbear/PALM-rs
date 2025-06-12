// Answer 0

#[test]
fn test_into_values_non_empty_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestMap {
        pub fn new() -> Self {
            TestMap { entries: vec![] }
        }
        
        pub fn add_entry(&mut self, key: i32, value: String) {
            self.entries.push(Bucket { hash: 0, key, value });
        }
        
        pub fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }
        
        pub fn into_values(self) -> IntoValues<i32, String> {
            IntoValues::new(self.into_entries())
        }
    }

    let mut map = TestMap::new();
    map.add_entry(1, "value1".to_string());
    map.add_entry(2, "value2".to_string());
    
    let values_iterator = map.into_values();
    let collected_values: Vec<String> = values_iterator.iter.collect();

    assert_eq!(collected_values, vec!["value1", "value2"]);
}

#[test]
fn test_into_values_empty_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestMap {
        pub fn new() -> Self {
            TestMap { entries: vec![] }
        }
        
        pub fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.entries
        }
        
        pub fn into_values(self) -> IntoValues<i32, String> {
            IntoValues::new(self.into_entries())
        }
    }

    let map = TestMap::new();
    
    let values_iterator = map.into_values();
    let collected_values: Vec<String> = values_iterator.iter.collect();

    assert_eq!(collected_values, vec![]);
}

