// Answer 0


struct TestStruct {
    entries: Vec<i32>,
}

impl TestStruct {
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn erase_indices(&mut self, start: usize, end: usize) {
        self.entries.drain(start..end);
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.erase_indices(len, self.entries.len());
            self.entries.truncate(len);
        }
    }
}

#[test]
fn test_truncate_no_change() {
    let mut test_struct = TestStruct {
        entries: vec![1, 2, 3, 4, 5],
    };
    let initial_length = test_struct.len();
    
    test_struct.truncate(initial_length);
    
    assert_eq!(test_struct.len(), initial_length);
    assert_eq!(test_struct.entries, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_truncate_empty() {
    let mut test_struct = TestStruct {
        entries: Vec::new(),
    };
    
    test_struct.truncate(0);
    
    assert_eq!(test_struct.len(), 0);
    assert_eq!(test_struct.entries, Vec::new());
}

#[test]
fn test_truncate_single_element() {
    let mut test_struct = TestStruct {
        entries: vec![42],
    };
    
    test_struct.truncate(1);
    
    assert_eq!(test_struct.len(), 1);
    assert_eq!(test_struct.entries, vec![42]);
}


