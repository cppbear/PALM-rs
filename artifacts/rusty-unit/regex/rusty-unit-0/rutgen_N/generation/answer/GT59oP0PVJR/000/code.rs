// Answer 0

#[derive(Default)]
struct MyStruct {
    version: usize,
}

impl MyStruct {
    fn clear(&mut self) {
        self.version += 1;
    }
}

#[test]
fn test_clear_increments_version() {
    let mut my_struct = MyStruct::default();
    assert_eq!(my_struct.version, 0);
    
    my_struct.clear();
    assert_eq!(my_struct.version, 1);
    
    my_struct.clear();
    assert_eq!(my_struct.version, 2);
} 

#[test]
fn test_clear_multiple_calls() {
    let mut my_struct = MyStruct::default();
    
    for _ in 0..10 {
        my_struct.clear();
    }
    
    assert_eq!(my_struct.version, 10);
} 

#[test]
fn test_clear_initial_state() {
    let mut my_struct = MyStruct::default();
    assert_eq!(my_struct.version, 0);
    my_struct.clear();
    assert_eq!(my_struct.version, 1);
}

