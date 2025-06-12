// Answer 0

#[test]
fn test_advance_mut_a_rem_zero() {
    struct TestBuf {
        remaining: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Placeholder implementation
            unimplemented!()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            // Decrement the remaining count without going below zero
            if cnt <= self.remaining {
                self.remaining -= cnt;
            } else {
                self.remaining = 0;
            }
        }
    }

    let buf_a = TestBuf { remaining: 0 };
    let mut buf_b = TestBuf { remaining: 10 }; // b has remaining bytes
    let mut chain = Chain { a: buf_a, b: buf_b };

    unsafe {
        chain.advance_mut(5);
    }
    
    // Check that buffer b has now 5 remaining since a had none to consume
    assert_eq!(chain.b.remaining_mut(), 5);
}

