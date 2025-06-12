// Answer 0

#[test]
fn test_get_full_found() {
    struct TestMap {
        data: Vec<(String, ())>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { data: vec![] }
        }

        fn get_full<Q>(&self, value: &Q) -> Option<(usize, &String)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.data.iter().enumerate().find_map(|(i, (k, _))| {
                if value.eq(k) {
                    Some((i, k))
                } else {
                    None
                }
            })
        }
    }

    let mut map = TestMap::new();
    map.data.push(("key1".to_string(), ()));
    map.data.push(("key2".to_string(), ()));

    let result = map.get_full(&"key1".to_string());
    assert_eq!(result, Some((0, &"key1".to_string())));
}

#[test]
fn test_get_full_not_found() {
    struct TestMap {
        data: Vec<(String, ())>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { data: vec![] }
        }

        fn get_full<Q>(&self, value: &Q) -> Option<(usize, &String)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.data.iter().enumerate().find_map(|(i, (k, _))| {
                if value.eq(k) {
                    Some((i, k))
                } else {
                    None
                }
            })
        }
    }

    let mut map = TestMap::new();
    map.data.push(("key1".to_string(), ()));
    
    let result = map.get_full(&"key3".to_string());
    assert_eq!(result, None);
}

