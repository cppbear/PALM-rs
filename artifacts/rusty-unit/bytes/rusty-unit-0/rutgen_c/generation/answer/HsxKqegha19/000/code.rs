// Answer 0

#[test]
fn test_get_mut() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.reserve(cnt);
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }
        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(&vec![val; cnt]);
        }
        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }
        fn put_i8(&mut self, n: i8) {
            self.data.push(n as u8);
        }
        fn put_u16(&mut self, n: u16) {
            self.data.extend_from_slice(&n.to_le_bytes());
        }
        // ... other methods would be similarly defined ...
        fn put_f64_ne(&mut self, n: f64) {
            unimplemented!()
        }
    }

    let mut buf = TestBuf {
        data: Vec::new(),
        capacity: 1024,
    };
    let mut writer = Writer { buf };

    let mutable_buf = writer.get_mut();
    mutable_buf.put_u8(10);
    
    assert_eq!(1, mutable_buf.data.len());
    assert_eq!(10, mutable_buf.data[0]);
}

#[test]
fn test_get_mut_with_capacity() {
    struct TestBuf {
        data: Vec<u8>,
        capacity: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.reserve(cnt);
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }
        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }
        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(&vec![val; cnt]);
        }
        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }
        fn put_i8(&mut self, n: i8) {
            self.data.push(n as u8);
        }
        fn put_u16(&mut self, n: u16) {
            self.data.extend_from_slice(&n.to_le_bytes());
        }
        // ... other methods would be similarly defined ...
        fn put_f64_ne(&mut self, n: f64) {
            unimplemented!()
        }
    }

    let mut buf = TestBuf {
        data: Vec::new(),
        capacity: 1024,
    };
    let mut writer = Writer { buf };

    assert_eq!(1024, writer.get_ref().capacity);
    writer.get_mut().put_bytes(5, 5);
    
    assert_eq!(5, writer.get_mut().data.len());
    assert_eq!(5, writer.get_mut().data[0]);
}

