// Answer 0

#[test]
fn test_try_reserve_with_exact_capacity() {
    struct TestIndices {
        capacity: usize,
        length: usize,
    }

    impl TestIndices {
        fn try_reserve(&mut self, additional: usize, _hash: usize) -> Result<(), ()> {
            if additional > self.capacity - self.length {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    struct TestEntries {
        capacity: usize,
        length: usize,
    }

    impl TestEntries {
        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.length
        }
    }

    struct TestStruct {
        indices: TestIndices,
        entries: TestEntries,
    }

    impl TestStruct {
        fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
            if self.indices.try_reserve(additional, 0).is_err() {
                return Err(());
            }
            if additional > self.entries.capacity() - self.entries.len() {
                return Err(());
            }
            Ok(())
        }
    }

    let mut test_struct = TestStruct {
        indices: TestIndices { capacity: 10, length: 8 },
        entries: TestEntries { capacity: 10, length: 8 },
    };
    
    let additional = test_struct.entries.capacity() - test_struct.entries.len(); // This will be 2.
    
    let result = test_struct.try_reserve(additional);
    
    assert_eq!(result, Ok(()));
}

