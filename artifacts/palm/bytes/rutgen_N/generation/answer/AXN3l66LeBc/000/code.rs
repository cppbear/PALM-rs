// Answer 0

#[test]
fn test_len() {
    struct TestBufMut<'a>(&'a mut [u8]);
    
    impl<'a> TestBufMut<'a> {
        pub fn chunk_mut(slice: &'a mut [u8]) -> TestBufMut<'a> {
            TestBufMut(slice)
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    let mut data = [0, 1, 2];
    let mut slice = &mut data[..];
    let len = TestBufMut::chunk_mut(&mut slice).len();

    assert_eq!(len, 3);
}

#[test]
fn test_len_empty_slice() {
    struct TestBufMut<'a>(&'a mut [u8]);
    
    impl<'a> TestBufMut<'a> {
        pub fn chunk_mut(slice: &'a mut [u8]) -> TestBufMut<'a> {
            TestBufMut(slice)
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    let mut data: [u8; 0] = [];
    let mut slice = &mut data[..];
    let len = TestBufMut::chunk_mut(&mut slice).len();

    assert_eq!(len, 0);
}

