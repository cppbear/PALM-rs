// Answer 0

#[test]
fn test_search_found() {
    struct TestMap {
        table: HashMap<u64, (&'static str, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut table = HashMap::new();
            table.insert(1, ("key1", "value1"));
            table.insert(2, ("key2", "value2"));
            TestMap { table }
        }

        fn search<F>(&self, hash: u64, mut is_match: F) -> Option<(&'static str, &'static str)>
        where
            F: FnMut(&&str) -> bool,
        {
            self.table.get(&hash).and_then(|(k, v)| {
                if is_match(k) {
                    Some((*k, *v))
                } else {
                    None
                }
            })
        }
    }

    let map = TestMap::new();
    let result = map.search(1, |key| key == "key1");
    assert_eq!(result, Some(("key1", "value1")));
}

#[test]
fn test_search_not_found() {
    struct TestMap {
        table: HashMap<u64, (&'static str, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut table = HashMap::new();
            table.insert(1, ("key1", "value1"));
            table.insert(2, ("key2", "value2"));
            TestMap { table }
        }

        fn search<F>(&self, hash: u64, mut is_match: F) -> Option<(&'static str, &'static str)>
        where
            F: FnMut(&&str) -> bool,
        {
            self.table.get(&hash).and_then(|(k, v)| {
                if is_match(k) {
                    Some((*k, *v))
                } else {
                    None
                }
            })
        }
    }

    let map = TestMap::new();
    let result = map.search(3, |key| key == "key3");
    assert_eq!(result, None);
}

#[test]
fn test_search_no_match() {
    struct TestMap {
        table: HashMap<u64, (&'static str, &'static str)>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut table = HashMap::new();
            table.insert(1, ("key1", "value1"));
            table.insert(2, ("key2", "value2"));
            TestMap { table }
        }

        fn search<F>(&self, hash: u64, mut is_match: F) -> Option<(&'static str, &'static str)>
        where
            F: FnMut(&&str) -> bool,
        {
            self.table.get(&hash).and_then(|(k, v)| {
                if is_match(k) {
                    Some((*k, *v))
                } else {
                    None
                }
            })
        }
    }

    let map = TestMap::new();
    let result = map.search(1, |key| key == "key2");
    assert_eq!(result, None);
}

