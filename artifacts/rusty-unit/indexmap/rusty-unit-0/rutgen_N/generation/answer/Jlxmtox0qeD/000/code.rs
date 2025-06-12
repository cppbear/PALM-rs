// Answer 0

#[test]
fn test_remove() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        fn swap_remove(&mut self, index: usize) -> String {
            let last_index = self.data.len() - 1;
            if index < last_index {
                self.data.swap(index, last_index);
            }
            self.data.pop().unwrap().1
        }

        fn remove(&mut self, key: i32) -> Option<String> {
            if let Some(pos) = self.data.iter().position(|(k, _)| *k == key) {
                Some(self.swap_remove(pos))
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    assert_eq!(map.remove(2), Some("two".to_string()));
    assert_eq!(map.data.len(), 2);
    assert_eq!(map.data[0].1, "one");
    assert_eq!(map.data[1].1, "three");
    
    assert_eq!(map.remove(4), None);
}

