// Answer 0

#[test]
fn test_writer_write_with_sufficient_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.position < self.data.len()
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            let len = src.len();
            self.data[self.position..self.position + len].copy_from_slice(src);
            self.position += len;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 10],
        position: 0,
    };
    let mut writer = Writer { buf };

    let result = writer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_writer_write_with_partial_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.position < self.data.len()
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            let len = src.len();
            self.data[self.position..self.position + len].copy_from_slice(src);
            self.position += len;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 3],
        position: 0,
    };
    let mut writer = Writer { buf };

    let result = writer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Ok(3));
}

#[test]
fn test_writer_write_with_empty_src() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.position < self.data.len()
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            let len = src.len();
            self.data[self.position..self.position + len].copy_from_slice(src);
            self.position += len;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 10],
        position: 0,
    };
    let mut writer = Writer { buf };

    let result = writer.write(&[]);
    assert_eq!(result, Ok(0));
} 

#[test]
#[should_panic]
fn test_writer_write_panic_on_out_of_bounds() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.position < self.data.len()
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            let len = src.len();
            self.data[self.position..self.position + len].copy_from_slice(src);
            self.position += len;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 5],
        position: 5,
    };
    let mut writer = Writer { buf };

    writer.write(&[1, 2, 3]);
}

