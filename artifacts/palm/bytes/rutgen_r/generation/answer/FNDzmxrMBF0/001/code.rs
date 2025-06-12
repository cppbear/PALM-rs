// Answer 0

#[test]
fn test_put_slice_with_insufficient_capacity() {
    struct BufMut<'a>(&'a mut [u8]);
    
    impl<'a> BufMut<'a> {
        fn remaining_mut(&self) -> usize {
            self.0.len()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let len = self.0.len();
            self.0 = &mut self.0[cnt..len];
        }

        fn put_slice(&mut self, mut src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Insufficient capacity");
            }

            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());

                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];

                // SAFETY: We just initialized `cnt` bytes in `self`.
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buffer = [0; 3];
    let mut buf = BufMut(&mut buffer[..]);
    
    let result = std::panic::catch_unwind(|| {
        buf.put_slice(b"hello");
    });

    assert!(result.is_err());
}

#[test]
fn test_put_slice_exact_capacity() {
    struct BufMut<'a>(&'a mut [u8]);
    
    impl<'a> BufMut<'a> {
        fn remaining_mut(&self) -> usize {
            self.0.len()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let len = self.0.len();
            self.0 = &mut self.0[cnt..len];
        }

        fn put_slice(&mut self, src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Insufficient capacity");
            }

            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());

                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];

                // SAFETY: We just initialized `cnt` bytes in `self`.
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buffer = [0; 5];
    {
        let mut buf = BufMut(&mut buffer[..]);
        buf.put_slice(b"hello");
    }
    
    assert_eq!(buffer, b"hello".to_vec().as_slice());
}

#[test]
fn test_put_slice_partial_capacity() {
    struct BufMut<'a>(&'a mut [u8]);
    
    impl<'a> BufMut<'a> {
        fn remaining_mut(&self) -> usize {
            self.0.len()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.0
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            let len = self.0.len();
            self.0 = &mut self.0[cnt..len];
        }

        fn put_slice(&mut self, mut src: &[u8]) {
            if self.remaining_mut() < src.len() {
                panic!("Insufficient capacity");
            }

            while !src.is_empty() {
                let dst = self.chunk_mut();
                let cnt = usize::min(src.len(), dst.len());

                dst[..cnt].copy_from_slice(&src[..cnt]);
                src = &src[cnt..];

                // SAFETY: We just initialized `cnt` bytes in `self`.
                unsafe { self.advance_mut(cnt) };
            }
        }
    }

    let mut buffer = [0; 4];
    {
        let mut buf = BufMut(&mut buffer[..]);
        buf.put_slice(b"he");
    }
    
    assert_eq!(&buffer[..2], b"he".as_slice());
    assert_eq!(&buffer[2..], &[0; 2]);  // Remaining part is zeroed out
}

