// Answer 0

#[test]
fn test_fmt_on_empty_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        inner: HashMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                inner: HashMap::new(),
            }
        }

        fn iter(&self) -> std::collections::hash_map::Iter<i32, String> {
            self.inner.iter()
        }
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.iter()).finish()
        }
    }

    let map = TestMap::new();
    let output = format!("{:?}", map);
    assert_eq!(output, "{}");
}

#[test]
fn test_fmt_on_non_empty_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        inner: HashMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut inner = HashMap::new();
            inner.insert(1, String::from("one"));
            inner.insert(2, String::from("two"));
            TestMap { inner }
        }

        fn iter(&self) -> std::collections::hash_map::Iter<i32, String> {
            self.inner.iter()
        }
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.iter()).finish()
        }
    }

    let map = TestMap::new();
    let output = format!("{:?}", map);
    assert!(output.contains("1"));
    assert!(output.contains("one"));
    assert!(output.contains("2"));
    assert!(output.contains("two"));
}

