// Answer 0

#[test]
fn test_advance_unchecked_valid_count() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: usize,
    }

    impl BytesMut {
        fn new(len: usize, cap: usize) -> Self {
            // For simplicity, we're using a simple calculation for ptr
            let ptr = 0 as *mut u8; // Dummy pointer for the example
            Self { ptr, len, cap, kind: 1 }
        }

        fn get_vec_pos(&self) -> usize {
            // Dummy implementation, since we don't care about exact values in this test
            0
        }

        fn set_vec_pos(&mut self, pos: usize) {
            // No operation, just for the sake of the trait methods
        }
        
        fn promote_to_shared(&mut self, _ref_count: usize) {
            // No operation, just for testing purpose
        }

        fn kind(&self) -> usize {
            self.kind
        }
        
        unsafe fn advance_unchecked(&mut self, count: usize) {
            if count == 0 {
                return;
            }
            let kind = self.kind();
            let pos = self.get_vec_pos() + count;
            
            if pos <= 134217727 {
                self.set_vec_pos(pos);
            } else {
                self.promote_to_shared(1);
            }
            
            self.ptr = (self.ptr as usize + count) as *mut u8;
            self.len = self.len.checked_sub(count).unwrap_or(0);
            self.cap -= count;
        }
    }

    let mut bytes_mut = BytesMut::new(10, 10);
    
    unsafe {
        bytes_mut.advance_unchecked(1);
        assert_eq!(bytes_mut.len, 9);
        assert_eq!(bytes_mut.cap, 9);

        bytes_mut.advance_unchecked(5);
        assert_eq!(bytes_mut.len, 4);
        assert_eq!(bytes_mut.cap, 4);

        bytes_mut.advance_unchecked(4);
        assert_eq!(bytes_mut.len, 0);
        assert_eq!(bytes_mut.cap, 0);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_overflow() {
    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        kind: usize,
    }

    impl BytesMut {
        fn new(len: usize, cap: usize) -> Self {
            let ptr = 0 as *mut u8; // Dummy pointer for the example
            Self { ptr, len, cap, kind: 1 }
        }

        fn get_vec_pos(&self) -> usize {
            0
        }

        fn set_vec_pos(&mut self, pos: usize) {
            // No operation
        }

        fn promote_to_shared(&mut self, _ref_count: usize) {
            // No operation
        }

        fn kind(&self) -> usize {
            self.kind
        }

        unsafe fn advance_unchecked(&mut self, count: usize) {
            if count == 0 {
                return;
            }
            let kind = self.kind();

            let pos = self.get_vec_pos() + count;
            if pos <= 134217727 {
                self.set_vec_pos(pos);
            } else {
                self.promote_to_shared(1);
            }

            self.ptr = (self.ptr as usize + count) as *mut u8;
            self.len = self.len.checked_sub(count).unwrap_or(0);
            self.cap -= count;
        }
    }

    let mut bytes_mut = BytesMut::new(10, 10);

    unsafe {
        bytes_mut.advance_unchecked(11); // Try to advance more than capacity.
    }
}

