// Answer 0

#[test]
fn test_index_within_bounds() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                )
            })
        }
    }

    let test_struct = TestStruct::new(vec![10, 20, 30]);

    assert_eq!(test_struct.index(0), &10);
    assert_eq!(test_struct.index(1), &20);
    assert_eq!(test_struct.index(2), &30);
}

#[test]
#[should_panic(expected = "index out of bounds:")]
fn test_index_out_of_bounds_lower() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                )
            })
        }
    }

    let test_struct = TestStruct::new(vec![10, 20, 30]);
    test_struct.index(3);
}

#[test]
#[should_panic(expected = "index out of bounds:")]
fn test_index_out_of_bounds_higher() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.data.get(index)
        }

        fn index(&self, index: usize) -> &i32 {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                )
            })
        }
    }

    let test_struct = TestStruct::new(vec![10, 20, 30]);
    test_struct.index(5);
}

