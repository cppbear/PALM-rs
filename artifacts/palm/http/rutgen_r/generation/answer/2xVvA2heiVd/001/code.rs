// Answer 0

#[test]
fn test_find_with_some_value() {
    struct TestHeader {
        value: usize,
    }

    struct TestHeaderMap {
        headers: Vec<TestHeader>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { headers: Vec::new() }
        }
        
        fn add(&mut self, value: usize) {
            self.headers.push(TestHeader { value });
        }
        
        fn find(&self, header: &TestHeader) -> Option<(usize, usize)> {
            for (i, h) in self.headers.iter().enumerate() {
                if h.value == header.value {
                    return Some((i, h.value));
                }
            }
            None
        }
    }

    let header_map = {
        let mut map = TestHeaderMap::new();
        map.add(1);
        map.add(2);
        map.add(3);
        map
    };

    let header = TestHeader { value: 2 };

    let result = header_map.find(&header);
    assert_eq!(result, Some((1, 2)));
}

#[test]
fn test_find_with_none_value() {
    struct TestHeader {
        value: usize,
    }

    struct TestHeaderMap {
        headers: Vec<TestHeader>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { headers: Vec::new() }
        }
        
        fn add(&mut self, value: usize) {
            self.headers.push(TestHeader { value });
        }
        
        fn find(&self, header: &TestHeader) -> Option<(usize, usize)> {
            for (i, h) in self.headers.iter().enumerate() {
                if h.value == header.value {
                    return Some((i, h.value));
                }
            }
            None
        }
    }

    let header_map = {
        let mut map = TestHeaderMap::new();
        map.add(1);
        map.add(2);
        map.add(3);
        map
    };

    let header = TestHeader { value: 4 };

    let result = header_map.find(&header);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_find_with_panic_condition() {
    struct TestHeader {
        value: usize,
    }

    struct TestHeaderMap {
        headers: Vec<TestHeader>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { headers: Vec::new() }
        }
        
        fn add(&mut self, value: usize) {
            self.headers.push(TestHeader { value });
        }
        
        fn find(&self, _header: &TestHeader) -> Option<(usize, usize)> {
            panic!("Simulated panic condition");
        }
    }

    let header_map = TestHeaderMap::new();
    let header = TestHeader { value: 1 };

    header_map.find(&header);
}

