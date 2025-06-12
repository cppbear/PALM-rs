// Answer 0

#[derive(Debug)]
struct TestBufMut {
    data: Vec<u8>,
}

impl TestBufMut {
    fn new(data: Vec<u8>) -> Self {
        TestBufMut { data }
    }
}

struct MyStruct<T> {
    inner: T,
}

impl<T> MyStruct<T> {
    fn get_ref(&self) -> &T {
        &self.inner
    }
}

#[test]
fn test_get_ref() {
    let buf_mut = TestBufMut::new(vec![1, 2, 3, 4, 5]);
    let my_struct = MyStruct { inner: buf_mut };

    let result = my_struct.get_ref();
    assert_eq!(result.data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_get_ref_empty() {
    let buf_mut = TestBufMut::new(vec![]);
    let my_struct = MyStruct { inner: buf_mut };

    let result = my_struct.get_ref();
    assert_eq!(result.data, vec![]);
}

