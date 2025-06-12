// Answer 0

#[test]
fn test_fmt_with_empty_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        map: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.map.iter()).finish()
        }
    }

    let empty_map = TestMap {
        map: HashMap::new(),
    };

    let result = format!("{:?}", empty_map);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_with_single_entry_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        map: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.map.iter()).finish()
        }
    }

    let single_entry_map = TestMap {
        map: {
            let mut m = HashMap::new();
            m.insert(1, 100);
            m
        },
    };

    let result = format!("{:?}", single_entry_map);
    assert_eq!(result, "{1: 100}");
}

#[test]
fn test_fmt_with_multiple_entries_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        map: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.map.iter()).finish()
        }
    }

    let multiple_entries_map = TestMap {
        map: {
            let mut m = HashMap::new();
            m.insert(1, 100);
            m.insert(2, 200);
            m.insert(3, 300);
            m
        },
    };

    let result = format!("{:?}", multiple_entries_map);
    assert_eq!(result, "{1: 100, 2: 200, 3: 300}");
}

#[should_panic]
fn test_fmt_with_panic_on_empty_format() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        map: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.map.iter()).finish()
        }
    }

    let empty_map = TestMap {
        map: HashMap::new(),
    };

    let result_panic = format!("{}", empty_map); // intentionally trying to format as non-debug
}

#[test]
fn test_fmt_with_large_map() {
    use std::fmt;
    use hashbrown::HashMap;

    struct TestMap {
        map: HashMap<i32, i32>,
    }

    impl fmt::Debug for TestMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.map.iter()).finish()
        }
    }

    let large_map = TestMap {
        map: {
            let mut m = HashMap::new();
            for i in 0..1000 {
                m.insert(i, i * 10);
            }
            m
        },
    };

    let result = format!("{:?}", large_map);
    let expected_prefix = "{";
    let expected_suffix = "}";
    assert!(result.starts_with(expected_prefix) && result.ends_with(expected_suffix));
}

