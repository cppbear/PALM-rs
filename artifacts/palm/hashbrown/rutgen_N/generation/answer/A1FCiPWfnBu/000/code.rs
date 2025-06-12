// Answer 0

#[test]
fn test_fmt_empty_map() {
    use hashbrown::HashMap;
    use std::fmt;

    struct TestMap {
        inner: HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, v)| v))
                .finish()
        }
    }

    let map = TestMap {
        inner: HashMap::new(),
    };

    let result = format!("{:?}", map);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_entry() {
    use hashbrown::HashMap;
    use std::fmt;

    struct TestMap {
        inner: HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, v)| v))
                .finish()
        }
    }

    let mut map = TestMap {
        inner: HashMap::new(),
    };
    map.inner.insert(1, "one".to_string());

    let result = format!("{:?}", map);
    assert_eq!(result, "[\"one\"]");
}

#[test]
fn test_fmt_multiple_entries() {
    use hashbrown::HashMap;
    use std::fmt;

    struct TestMap {
        inner: HashMap<i32, String>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(self.inner.iter().map(|(_, v)| v))
                .finish()
        }
    }

    let mut map = TestMap {
        inner: HashMap::new(),
    };
    map.inner.insert(1, "one".to_string());
    map.inner.insert(2, "two".to_string());
    map.inner.insert(3, "three".to_string());

    let result = format!("{:?}", map);
    assert_eq!(result, "[\"one\", \"two\", \"three\"]");
}

