// Answer 0

#[derive(Debug)]
struct MyBytesMut {
    data: Vec<u8>,
}

impl MyBytesMut {
    fn new(data: Vec<u8>) -> Self {
        MyBytesMut { data }
    }

    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

#[test]
fn test_deref_mut_empty() {
    let mut my_bytes = MyBytesMut::new(Vec::new());
    let result = my_bytes.deref_mut();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_deref_mut_non_empty() {
    let mut my_bytes = MyBytesMut::new(vec![1, 2, 3, 4]);
    let result = my_bytes.deref_mut();
    assert_eq!(result, &[1, 2, 3, 4]);
}

#[test]
fn test_deref_mut_modify() {
    let mut my_bytes = MyBytesMut::new(vec![5, 6, 7]);
    {
        let result = my_bytes.deref_mut();
        result[0] = 10;
    }
    assert_eq!(my_bytes.deref_mut(), &[10, 6, 7]);
}

#[test]
fn test_deref_mut_single_element() {
    let mut my_bytes = MyBytesMut::new(vec![255]);
    let result = my_bytes.deref_mut();
    assert_eq!(result, &[255]);
}

