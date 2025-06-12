// Answer 0

#[test]
fn test_binary_search_keys_existing_key() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 3],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: "a", value: 1 },
                    Bucket { hash: 0, key: "b", value: 2 },
                    Bucket { hash: 0, key: "c", value: 3 },
                ],
            }
        }
        
        fn as_slice(&self) -> Slice<&'static str, i32> {
            Slice { entries: self.entries }
        }
    }

    let map = TestMap::new();
    let slice = map.as_slice();
    assert_eq!(slice.binary_search_keys(&"b").unwrap(), 1);
}

#[test]
fn test_binary_search_keys_missing_key() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 3],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: "a", value: 1 },
                    Bucket { hash: 0, key: "b", value: 2 },
                    Bucket { hash: 0, key: "c", value: 3 },
                ],
            }
        }
        
        fn as_slice(&self) -> Slice<&'static str, i32> {
            Slice { entries: self.entries }
        }
    }

    let map = TestMap::new();
    let slice = map.as_slice();
    assert_eq!(slice.binary_search_keys(&"d").unwrap_err(), 3);
}

#[test]
fn test_binary_search_keys_first_key() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 3],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: "a", value: 1 },
                    Bucket { hash: 0, key: "b", value: 2 },
                    Bucket { hash: 0, key: "c", value: 3 },
                ],
            }
        }
        
        fn as_slice(&self) -> Slice<&'static str, i32> {
            Slice { entries: self.entries }
        }
    }

    let map = TestMap::new();
    let slice = map.as_slice();
    assert_eq!(slice.binary_search_keys(&"a").unwrap(), 0);
}

#[test]
fn test_binary_search_keys_last_key() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 3],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: "a", value: 1 },
                    Bucket { hash: 0, key: "b", value: 2 },
                    Bucket { hash: 0, key: "c", value: 3 },
                ],
            }
        }
        
        fn as_slice(&self) -> Slice<&'static str, i32> {
            Slice { entries: self.entries }
        }
    }

    let map = TestMap::new();
    let slice = map.as_slice();
    assert_eq!(slice.binary_search_keys(&"c").unwrap(), 2);
}

#[test]
fn test_binary_search_keys_empty_slice() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 0],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [],
            }
        }
        
        fn as_slice(&self) -> Slice<&'static str, i32> {
            Slice { entries: self.entries }
        }
    }

    let map = TestMap::new();
    let slice = map.as_slice();
    assert_eq!(slice.binary_search_keys(&"a").unwrap_err(), 0);
}

