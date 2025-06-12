// Answer 0

#[test]
fn test_get_index_valid_index() {
    struct TestStruct {
        entries: Vec<(usize, usize)>
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                entries: vec![(0, 1), (1, 2), (2, 3)]
            }
        }

        fn as_entries(&self) -> &Vec<(usize, usize)> {
            &self.entries
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index(&self, index: usize) -> Option<&(usize, usize)> {
            self.as_entries().get(index)
        }
    }

    let test_struct = TestStruct::new();
    assert_eq!(test_struct.get_index(0), Some(&(0, 1)));
    assert_eq!(test_struct.get_index(1), Some(&(1, 2)));
    assert_eq!(test_struct.get_index(2), Some(&(2, 3)));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestStruct {
        entries: Vec<(usize, usize)>
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                entries: vec![(0, 1), (1, 2), (2, 3)]
            }
        }

        fn as_entries(&self) -> &Vec<(usize, usize)> {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index(&self, index: usize) -> Option<&(usize, usize)> {
            self.as_entries().get(index)
        }
    }

    let test_struct = TestStruct::new();
    assert_eq!(test_struct.get_index(3), None);
}

#[test]
fn test_get_index_empty() {
    struct TestStruct {
        entries: Vec<(usize, usize)>
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                entries: vec![]
            }
        }

        fn as_entries(&self) -> &Vec<(usize, usize)> {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index(&self, index: usize) -> Option<&(usize, usize)> {
            self.as_entries().get(index)
        }
    }

    let test_struct = TestStruct::new();
    assert_eq!(test_struct.get_index(0), None);
}

