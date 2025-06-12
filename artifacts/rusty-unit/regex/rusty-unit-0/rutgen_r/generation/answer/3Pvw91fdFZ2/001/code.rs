// Answer 0

#[test]
fn test_index_with_valid_group() {
    struct TestRegex {
        groups: Vec<String>,
    }

    impl TestRegex {
        fn get(&self, i: usize) -> Option<&String> {
            self.groups.get(i)
        }
    }

    let regex = TestRegex {
        groups: vec!["group0".to_string(), "group1".to_string(), "group2".to_string()],
    };

    assert_eq!(regex.index(0), "group0");
    assert_eq!(regex.index(1), "group1");
    assert_eq!(regex.index(2), "group2");
}

#[test]
#[should_panic(expected = "no group at index '3'")]
fn test_index_with_out_of_bounds() {
    struct TestRegex {
        groups: Vec<String>,
    }

    impl TestRegex {
        fn get(&self, i: usize) -> Option<&String> {
            self.groups.get(i)
        }
    }

    let regex = TestRegex {
        groups: vec!["group0".to_string(), "group1".to_string()],
    };

    regex.index(3);
}

#[test]
#[should_panic(expected = "no group at index '0'")]
fn test_index_with_empty_groups() {
    struct TestRegex {
        groups: Vec<String>,
    }

    impl TestRegex {
        fn get(&self, i: usize) -> Option<&String> {
            self.groups.get(i)
        }
    }

    let regex = TestRegex {
        groups: vec![],
    };

    regex.index(0);
}

