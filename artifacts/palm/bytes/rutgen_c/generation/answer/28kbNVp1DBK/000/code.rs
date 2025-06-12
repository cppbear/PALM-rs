// Answer 0

#[test]
fn test_remaining_mut() {
    struct TestBufMut {
        data: Vec<core::mem::MaybeUninit<u8>>,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.truncate(self.data.len().saturating_sub(cnt));
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len().min(self.remaining_mut());
            self.data.splice(..cnt, src.iter().map(|&x| core::mem::MaybeUninit::new(x)));
        }

        fn put_bytes(&mut self, _: u8, _: usize) {}
    }

    let buf = TestBufMut {
        data: vec![core::mem::MaybeUninit::uninit(); 10],
    };

    assert_eq!(buf.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_after_advance() {
    struct TestBufMut {
        data: Vec<core::mem::MaybeUninit<u8>>,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.truncate(self.data.len().saturating_sub(cnt));
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            let cnt = src.len().min(self.remaining_mut());
            self.data.splice(..cnt, src.iter().map(|&x| core::mem::MaybeUninit::new(x)));
        }

        fn put_bytes(&mut self, _: u8, _: usize) {}
    }

    let mut buf = TestBufMut {
        data: vec![core::mem::MaybeUninit::uninit(); 10],
    };

    unsafe {
        buf.advance_mut(5);
    }
    
    assert_eq!(buf.remaining_mut(), 5);
}

