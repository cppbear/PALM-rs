// Answer 0

#[test]
fn test_end_with_remaining_elements() {
    struct TestDeserializer {
        iter: std::vec::IntoIter<i32>,
        count: usize,
    }

    impl TestDeserializer {
        fn new(elements: Vec<i32>, count: usize) -> Self {
            TestDeserializer {
                iter: elements.into_iter(),
                count,
            }
        }

        fn end(self) -> Result<(), &'static str> {
            let remaining = self.iter.count();
            if remaining == 0 {
                Ok(())
            } else {
                Err("Additional elements remain.")
            }
        }
    }

    let elements = vec![1, 2, 3]; // non-empty vector to simulate remaining elements
    let deserializer = TestDeserializer::new(elements, 0); // count is 0
    assert_eq!(deserializer.end(), Err("Additional elements remain."));
}

#[test]
fn test_end_with_no_remaining_elements() {
    struct TestDeserializer {
        iter: std::vec::IntoIter<i32>,
        count: usize,
    }

    impl TestDeserializer {
        fn new(elements: Vec<i32>, count: usize) -> Self {
            TestDeserializer {
                iter: elements.into_iter(),
                count,
            }
        }

        fn end(self) -> Result<(), &'static str> {
            let remaining = self.iter.count();
            if remaining == 0 {
                Ok(())
            } else {
                Err("Additional elements remain.")
            }
        }
    }

    let elements = vec![]; // empty vector to simulate no remaining elements
    let deserializer = TestDeserializer::new(elements, 0); // count is 0
    assert_eq!(deserializer.end(), Ok(()));
}

