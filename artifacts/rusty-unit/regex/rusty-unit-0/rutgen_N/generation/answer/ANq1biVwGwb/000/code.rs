// Answer 0

#[derive(Debug)]
struct MyStruct {
    v: Vec<u8>,
}

impl MyStruct {
    fn new(data: Vec<u8>) -> Self {
        MyStruct { v: data }
    }

    fn deref(&self) -> &Vec<u8> {
        &self.v
    }
}

#[test]
fn test_deref() {
    let data = vec![1, 2, 3, 4];
    let my_struct = MyStruct::new(data);
    assert_eq!(*my_struct.deref(), vec![1, 2, 3, 4]);
}

#[test]
fn test_deref_empty() {
    let data = vec![];
    let my_struct = MyStruct::new(data);
    assert_eq!(*my_struct.deref(), vec![]);
}

#[test]
fn test_deref_single_element() {
    let data = vec![42];
    let my_struct = MyStruct::new(data);
    assert_eq!(*my_struct.deref(), vec![42]);
}

