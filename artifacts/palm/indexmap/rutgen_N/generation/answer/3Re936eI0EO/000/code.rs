// Answer 0

#[test]
fn test_fmt_empty_map() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestMap {
        map: IndexMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: IndexMap::new(),
            }
        }
    }

    let test_map = TestMap::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_map.map);
    assert!(result.is_ok());
    assert_eq!(buffer, "{}");
}

#[test]
fn test_fmt_non_empty_map() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestMap {
        map: IndexMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, 10);
            map.insert(2, 20);
            TestMap { map }
        }
    }

    let test_map = TestMap::new();
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_map.map);
    assert!(result.is_ok());
    assert_eq!(buffer, "{1: 10, 2: 20}");
}

