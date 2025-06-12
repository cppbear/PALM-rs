// Answer 0

#[test]
fn test_retain_with_some_elements_kept() {
    struct TestMap {
        core: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { core: vec![] }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.push((key, value));
        }

        fn retain_in_order<F>(&mut self, mut keep: F)
        where
            F: FnMut(&i32, &mut String) -> bool,
        {
            self.core.retain(|(k, v)| keep(k, v));
        }
        
        fn get_elements(&self) -> &Vec<(i32, String)> {
            &self.core
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|k, v| *k != 2);

    assert_eq!(map.get_elements(), &vec![(1, "one".to_string()), (3, "three".to_string())]);
}

#[test]
fn test_retain_with_no_elements_kept() {
    struct TestMap {
        core: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { core: vec![] }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.core.push((key, value));
        }

        fn retain_in_order<F>(&mut self, mut keep: F)
        where
            F: FnMut(&i32, &mut String) -> bool,
        {
            self.core.retain(|(k, v)| keep(k, v));
        }

        fn get_elements(&self) -> &Vec<(i32, String)> {
            &self.core
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|_k, _v| false);

    assert_eq!(map.get_elements(), &vec![]);
}

#[test]
fn test_retain_with_all_elements_kept() {
    struct TestMap {
        core: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { core: vec![] }
        }
        
        fn insert(&mut self, key: i32, value: String) {
            self.core.push((key, value));
        }

        fn retain_in_order<F>(&mut self, mut keep: F)
        where
            F: FnMut(&i32, &mut String) -> bool,
        {
            self.core.retain(|(k, v)| keep(k, v));
        }
        
        fn get_elements(&self) -> &Vec<(i32, String)> {
            &self.core
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|_k, _v| true);

    assert_eq!(map.get_elements(), &vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]);
}

