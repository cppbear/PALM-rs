// Answer 0

#[derive(Debug)]
struct TestStruct {
    data: Vec<i32>,
}

impl TestStruct {
    fn new(data: Vec<i32>) -> Self {
        TestStruct { data }
    }

    fn clone_from_impl(&mut self, source: &Self) {
        self.data.clear();
        self.data.extend_from_slice(&source.data);
    }
}

unsafe fn clone_from_spec(&mut self, source: &Self) {
    self.clone_from_impl(source);
}

#[test]
fn test_clone_from_spec() {
    let mut destination = TestStruct::new(vec![1, 2, 3]);
    let source = TestStruct::new(vec![4, 5, 6]);
    
    unsafe {
        clone_from_spec(&mut destination, &source);
    }
    
    assert_eq!(destination.data, vec![4, 5, 6]);
}

#[test]
fn test_clone_from_spec_empty_source() {
    let mut destination = TestStruct::new(vec![1, 2, 3]);
    let source = TestStruct::new(vec![]);
    
    unsafe {
        clone_from_spec(&mut destination, &source);
    }
    
    assert_eq!(destination.data, vec![]);
}

#[test]
fn test_clone_from_spec_clone_to_empty() {
    let mut destination = TestStruct::new(vec![]);
    let source = TestStruct::new(vec![7, 8, 9]);
    
    unsafe {
        clone_from_spec(&mut destination, &source);
    }
    
    assert_eq!(destination.data, vec![7, 8, 9]);
}

