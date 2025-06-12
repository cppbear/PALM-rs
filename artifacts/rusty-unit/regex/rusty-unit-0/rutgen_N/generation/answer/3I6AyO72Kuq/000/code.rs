// Answer 0

#[derive(Debug)]
struct MyStruct(Vec<u8>);

impl MyStruct {
    fn new(data: Vec<u8>) -> Self {
        MyStruct(data)
    }

    fn deref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

#[test]
fn test_deref_non_empty() {
    let my_struct = MyStruct::new(vec![1, 2, 3]);
    let result = my_struct.deref();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_deref_empty() {
    let my_struct = MyStruct::new(vec![]);
    let result = my_struct.deref();
    assert_eq!(result, &[]);
}

