// Answer 0

#[test]
fn test_remove_extra_value_valid_index() {
    struct TestStruct {
        extra_values: Vec<ExtraValue<i32>>,
    }

    impl TestStruct {
        fn raw_links(&self) -> &Vec<usize> {
            &self.extra_values.iter().map(|_| 0).collect() // dummy implementation
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<i32> {
            let raw_links = self.raw_links();
            remove_extra_value(raw_links, &mut self.extra_values, idx)
        }
    }

    let mut test_struct = TestStruct {
        extra_values: vec![ExtraValue(1), ExtraValue(2), ExtraValue(3)],
    };

    let removed_value = test_struct.remove_extra_value(1);
    assert_eq!(removed_value, ExtraValue(2));
    assert_eq!(test_struct.extra_values.len(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_remove_extra_value_index_out_of_bounds() {
    struct TestStruct {
        extra_values: Vec<ExtraValue<i32>>,
    }

    impl TestStruct {
        fn raw_links(&self) -> &Vec<usize> {
            &self.extra_values.iter().map(|_| 0).collect() // dummy implementation
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<i32> {
            let raw_links = self.raw_links();
            remove_extra_value(raw_links, &mut self.extra_values, idx)
        }
    }

    let mut test_struct = TestStruct {
        extra_values: vec![ExtraValue(1), ExtraValue(2), ExtraValue(3)],
    };

    test_struct.remove_extra_value(3); // This should panic as the index is out of bounds
}

#[test]
fn test_remove_extra_value_empty() {
    struct TestStruct {
        extra_values: Vec<ExtraValue<i32>>,
    }

    impl TestStruct {
        fn raw_links(&self) -> &Vec<usize> {
            &self.extra_values.iter().map(|_| 0).collect() // dummy implementation
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<i32> {
            let raw_links = self.raw_links();
            remove_extra_value(raw_links, &mut self.extra_values, idx)
        }
    }

    let mut test_struct = TestStruct {
        extra_values: Vec::new(),
    };

    // This will panic, but it should be captured in the test's environment
    let result = std::panic::catch_unwind(|| {
        test_struct.remove_extra_value(0);
    });

    assert!(result.is_err()); // Ensure that it did panic
}

