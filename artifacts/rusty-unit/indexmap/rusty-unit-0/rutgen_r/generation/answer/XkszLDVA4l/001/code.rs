// Answer 0

#[test]
fn test_split_off_at_len() {
    struct TestStruct {
        entries: Vec<i32>, // Assuming entries is a Vec for this example
    }
    
    impl TestStruct {
        fn split_off(&mut self, at: usize) -> Self {
            let len = self.entries.len();
            assert!(
                at <= len,
                "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
            );

            // Simulate erasure of indices (not implemented here for simplicity)
            self.entries.truncate(at); // Just truncating for example's sake
            let entries = self.entries.split_off(at);

            let mut indices = vec![0; entries.len()]; // Placeholder for indices
            indices.copy_from_slice(&entries); // Simple mock-up

            Self { entries, indices }
        }
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
    };
    let result = test_instance.split_off(test_instance.entries.len());

    assert_eq!(result.entries, vec![]);
    assert_eq!(result.indices, vec![1, 2, 3, 4, 5]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_off_at_out_of_bounds() {
    struct TestStruct {
        entries: Vec<i32>, 
    }
    
    impl TestStruct {
        fn split_off(&mut self, at: usize) -> Self {
            let len = self.entries.len();
            assert!(
                at <= len,
                "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
            );

            self.entries.truncate(at);
            let entries = self.entries.split_off(at);

            let mut indices = vec![0; entries.len()]; 
            indices.copy_from_slice(&entries); 

            Self { entries, indices }
        }
    }

    let mut test_instance = TestStruct {
        entries: vec![1, 2, 3],
    };
    test_instance.split_off(4);
}

