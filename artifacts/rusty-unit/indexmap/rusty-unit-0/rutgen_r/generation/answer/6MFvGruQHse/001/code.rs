// Answer 0

#[test]
fn test_replace_existing_value() {
    struct TestSet {
        data: std::collections::HashSet<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                data: std::collections::HashSet::new(),
            }
        }

        fn replace(&mut self, value: i32) -> Option<i32> {
            let exists = self.data.replace(value);
            exists
        }
    }

    let mut set = TestSet::new();
    set.data.insert(1);
    set.data.insert(2);

    assert_eq!(set.replace(1), None); // Replace existing value
    assert_eq!(set.replace(3), None); // Insert new value
    assert_eq!(set.replace(2), None); // Replace existing value
}

#[test]
fn test_replace_no_existing_value() {
    struct TestSet {
        data: std::collections::HashSet<String>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                data: std::collections::HashSet::new(),
            }
        }

        fn replace(&mut self, value: String) -> Option<String> {
            let exists = self.data.replace(value);
            exists
        }
    }

    let mut set = TestSet::new();

    assert_eq!(set.replace("test".to_string()), None); // Insert new value
    assert_eq!(set.replace("test".to_string()), None); // Already exists, replace
}

#[test]
fn test_replace_with_boundary_value() {
    struct TestSet {
        data: std::collections::HashSet<u32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                data: std::collections::HashSet::new(),
            }
        }

        fn replace(&mut self, value: u32) -> Option<u32> {
            let exists = self.data.replace(value);
            exists
        }
    }

    let mut set = TestSet::new();

    assert_eq!(set.replace(0), None); // Insert minimum value
    assert_eq!(set.replace(u32::MAX), None); // Insert maximum value
    assert_eq!(set.replace(0), Some(0)); // Replace existing minimum value
    assert_eq!(set.replace(u32::MAX), Some(u32::MAX)); // Replace existing maximum value
}

