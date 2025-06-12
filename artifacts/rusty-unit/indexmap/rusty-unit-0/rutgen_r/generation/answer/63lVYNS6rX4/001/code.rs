// Answer 0

#[test]
fn test_fmt_non_empty_set() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestSet {
        data: IndexMap<i32, i32>,
    }

    impl TestSet {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, 10);
            map.insert(2, 20);
            map.insert(3, 30);
            Self { data: map }
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.data.keys()
        }
    }

    let test_set = TestSet::new();
    let output = format!("{:?}", test_set);
    assert!(output.contains("1"));
    assert!(output.contains("2"));
    assert!(output.contains("3"));
}

#[test]
fn test_fmt_empty_set() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestSet {
        data: IndexMap<i32, i32>,
    }

    impl TestSet {
        fn new() -> Self {
            Self { data: IndexMap::new() }
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.data.keys()
        }
    }

    let test_set = TestSet::new();
    let output = format!("{:?}", test_set);
    assert_eq!(output, "{}", "Formatted output for an empty set should be empty.");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_invalid_format() {
    use std::fmt;
    use indexmap::IndexMap;

    struct TestSet {
        data: IndexMap<i32, i32>,
    }

    impl TestSet {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, 10);
            map.insert(2, 20);
            Self { data: map }
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.data.keys()
        }
    }

    let test_set = TestSet::new();
    // This should panic if we use an invalid formatting, simulating a panic case.
    let _ = format!("{:invalid}", test_set);
}

