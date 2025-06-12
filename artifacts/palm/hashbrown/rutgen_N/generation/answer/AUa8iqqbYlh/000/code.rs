// Answer 0

#[test]
fn test_fmt_empty_set() {
    use std::fmt;
    use hashbrown::HashSet;

    struct TestSet {
        set: HashSet<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                set: HashSet::new(),
            }
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, i32> {
            self.set.iter()
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    }

    let test_set = TestSet::new();
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_set);
    assert!(result.is_ok());
    assert_eq!(output, "{}");
}

#[test]
fn test_fmt_non_empty_set() {
    use std::fmt;
    use hashbrown::HashSet;

    struct TestSet {
        set: HashSet<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                set: HashSet::from([1, 2, 3]),
            }
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, i32> {
            self.set.iter()
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    }

    let test_set = TestSet::new();
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_set);
    assert!(result.is_ok());
    assert_eq!(output, "{1, 2, 3}"); // Output order may vary
}

