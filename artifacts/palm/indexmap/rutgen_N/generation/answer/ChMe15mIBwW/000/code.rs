// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct TestStruct {
        data: Vec<i32>,
    }
    
    impl TestStruct {
        fn new() -> Self {
            TestStruct { data: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if from < self.data.len() && to < self.data.len() {
                let item = self.data.remove(from);
                self.data.insert(to, item);
            }
        }
        
        fn borrow_mut(&mut self) -> &mut Self {
            self
        }
    }

    let mut test_struct = TestStruct::new();
    test_struct.move_index(1, 3);
    assert_eq!(test_struct.data, vec![1, 3, 4, 2, 5]);
}

#[test]
fn test_move_index_out_of_bounds() {
    struct TestStruct {
        data: Vec<i32>,
    }
    
    impl TestStruct {
        fn new() -> Self {
            TestStruct { data: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if from < self.data.len() && to < self.data.len() {
                let item = self.data.remove(from);
                self.data.insert(to, item);
            }
        }

        fn borrow_mut(&mut self) -> &mut Self {
            self
        }
    }
    
    let mut test_struct = TestStruct::new();
    test_struct.move_index(5, 1); // Out of bounds 'from' index
    assert_eq!(test_struct.data, vec![1, 2, 3, 4, 5]); // Should remain unchanged

    test_struct.move_index(1, 5); // Out of bounds 'to' index
    assert_eq!(test_struct.data, vec![1, 2, 3, 4, 5]); // Should remain unchanged
}

