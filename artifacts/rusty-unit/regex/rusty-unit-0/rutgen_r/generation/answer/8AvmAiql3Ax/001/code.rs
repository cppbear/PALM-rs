// Answer 0

#[test]
fn test_index_valid_group() {
    struct TestStruct {
        data: Vec<String>,
    }

    impl TestStruct {
        fn get(&self, i: usize) -> Option<&String> {
            self.data.get(i)
        }

        fn index(&self, i: usize) -> &[u8] {
            self.get(i).map(|m| m.as_bytes())
                .unwrap_or_else(|| panic!("no group at index '{}'", i))
        }
    }

    let test_struct = TestStruct {
        data: vec!["first".to_string(), "second".to_string()],
    };

    let result = test_struct.index(0);
    assert_eq!(result, b"first"); // Checking the first element

    let result = test_struct.index(1);
    assert_eq!(result, b"second"); // Checking the second element
}

#[test]
#[should_panic(expected = "no group at index '2'")]
fn test_index_panic_out_of_bounds() {
    struct TestStruct {
        data: Vec<String>,
    }

    impl TestStruct {
        fn get(&self, i: usize) -> Option<&String> {
            self.data.get(i)
        }

        fn index(&self, i: usize) -> &[u8] {
            self.get(i).map(|m| m.as_bytes())
                .unwrap_or_else(|| panic!("no group at index '{}'", i))
        }
    }

    let test_struct = TestStruct {
        data: vec!["first".to_string(), "second".to_string()],
    };

    test_struct.index(2); // This should panic
}

#[test]
#[should_panic(expected = "no group at index '1'")]
fn test_index_panic_empty_data() {
    struct TestStruct {
        data: Vec<String>,
    }

    impl TestStruct {
        fn get(&self, i: usize) -> Option<&String> {
            self.data.get(i)
        }

        fn index(&self, i: usize) -> &[u8] {
            self.get(i).map(|m| m.as_bytes())
                .unwrap_or_else(|| panic!("no group at index '{}'", i))
        }
    }

    let test_struct = TestStruct {
        data: Vec::new(),
    };

    test_struct.index(1); // This should panic
}

