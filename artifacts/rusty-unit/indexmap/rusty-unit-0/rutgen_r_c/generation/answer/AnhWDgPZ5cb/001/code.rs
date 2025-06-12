// Answer 0

#[test]
fn test_binary_search_keys_found() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 5],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: Default::default(), key: "a", value: 1 },
                    Bucket { hash: Default::default(), key: "b", value: 2 },
                    Bucket { hash: Default::default(), key: "c", value: 3 },
                    Bucket { hash: Default::default(), key: "d", value: 4 },
                    Bucket { hash: Default::default(), key: "e", value: 5 },
                ],
            }
        }

        fn slice(&self) -> &Slice<&'static str, i32> {
            unsafe { &*(self as *const _ as *const Slice<&'static str, i32>) }
        }
    }

    let map = TestMap::new();
    let result = map.slice().binary_search_keys(&"c");
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_keys_not_found() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 5],
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: Default::default(), key: "a", value: 1 },
                    Bucket { hash: Default::default(), key: "b", value: 2 },
                    Bucket { hash: Default::default(), key: "c", value: 3 },
                    Bucket { hash: Default::default(), key: "d", value: 4 },
                    Bucket { hash: Default::default(), key: "e", value: 5 },
                ],
            }
        }

        fn slice(&self) -> &Slice<&'static str, i32> {
            unsafe { &*(self as *const _ as *const Slice<&'static str, i32>) }
        }
    }

    let map = TestMap::new();
    let result = map.slice().binary_search_keys(&"f");
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_keys_empty() {
    struct TestMap {
        entries: [Bucket<&'static str, i32>; 0],
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: [] }
        }

        fn slice(&self) -> &Slice<&'static str, i32> {
            unsafe { &*(self as *const _ as *const Slice<&'static str, i32>) }
        }
    }

    let map = TestMap::new();
    let result = map.slice().binary_search_keys(&"a");
    assert_eq!(result, Err(0));
}

