// Answer 0

#[test]
fn test_into_inner_with_empty_writer() {
    struct DummyBufMut {
        data: Vec<u8>,
    }

    impl BufMut for DummyBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len()
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            !self.data.is_empty()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        // other BufMut methods are left unimplemented for brevity
        fn put_bytes(&mut self, _: u8, _: usize) {}
        fn put_u8(&mut self, _: u8) {}
        fn put_i8(&mut self, _: i8) {}
        fn put_u16(&mut self, _: u16) {}
        fn put_u16_le(&mut self, _: u16) {}
        fn put_u16_ne(&mut self, _: u16) {}
        // add stubs for remaining methods ...
    }

    let buf = DummyBufMut { data: Vec::new() };
    let writer = Writer { buf };

    assert_eq!(writer.into_inner().remaining_mut(), 0);
}

#[test]
fn test_into_inner_with_data_writer() {
    struct DummyBufMut {
        data: Vec<u8>,
    }

    impl BufMut for DummyBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len()
        }

        unsafe fn advance_mut(&mut self, _: usize) {}

        fn has_remaining_mut(&self) -> bool {
            !self.data.is_empty()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        // other BufMut methods are left unimplemented for brevity
        fn put_bytes(&mut self, _: u8, _: usize) {}
        fn put_u8(&mut self, _: u8) {}
        fn put_i8(&mut self, _: i8) {}
        fn put_u16(&mut self, _: u16) {}
        fn put_u16_le(&mut self, _: u16) {}
        fn put_u16_ne(&mut self, _: u16) {}
        // add stubs for remaining methods ...
    }

    let mut buf = DummyBufMut { data: Vec::new() };
    buf.put_slice(b"hello world");
    let writer = Writer { buf };

    let inner_buf = writer.into_inner();
    assert_eq!(inner_buf.data, b"hello world".to_vec());
    assert_eq!(inner_buf.remaining_mut(), 11);
}

