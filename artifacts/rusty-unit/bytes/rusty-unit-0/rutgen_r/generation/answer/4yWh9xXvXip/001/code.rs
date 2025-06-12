// Answer 0

#[test]
fn test_copy_to_bytes_a_rem_equals_len() {
    struct MockA {
        remaining: usize,
        data: Vec<u8>,
    }
    
    impl MockA {
        fn remaining(&self) -> usize {
            self.remaining
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            self.data.split_off(len).into()
        }
    }
    
    struct MockB {
        remaining: usize,
        data: Vec<u8>,
    }
    
    impl MockB {
        fn remaining(&self) -> usize {
            self.remaining
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            self.data.split_off(len).into()
        }
    }
    
    struct MockChain {
        a: MockA,
        b: MockB,
    }
    
    impl MockChain {
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let a_rem = self.a.remaining();
            if a_rem >= len {
                self.a.copy_to_bytes(len)
            } else if a_rem == 0 {
                self.b.copy_to_bytes(len)
            } else {
                assert!(
                    len - a_rem <= self.b.remaining(),
                    "`len` greater than remaining"
                );
                let mut ret = crate::BytesMut::with_capacity(len);
                ret.put(&mut self.a);
                ret.put((&mut self.b).take(len - a_rem));
                ret.freeze()
            }
        }
    }
    
    let a_data = vec![1, 2, 3, 4];
    let b_data = vec![5, 6, 7, 8];
    
    let mut chain = MockChain {
        a: MockA {
            remaining: 4,
            data: a_data,
        },
        b: MockB {
            remaining: 4,
            data: b_data,
        },
    };

    let result = chain.copy_to_bytes(4);
    
    assert_eq!(result.len(), 4);
    assert_eq!(result.as_ref(), &[1, 2, 3, 4]);
}

#[test]
#[should_panic(expected = "`len` greater than remaining")]
fn test_copy_to_bytes_panics() {
    struct MockA {
        remaining: usize,
    }
    
    impl MockA {
        fn remaining(&self) -> usize {
            self.remaining
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            // Implementation not needed for this test
            crate::Bytes::new()
        }
    }
    
    struct MockB {
        remaining: usize,
    }
    
    impl MockB {
        fn remaining(&self) -> usize {
            self.remaining
        }
        
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            // Implementation not needed for this test
            crate::Bytes::new()
        }
    }
    
    struct MockChain {
        a: MockA,
        b: MockB,
    }
    
    impl MockChain {
        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            let a_rem = self.a.remaining();
            if a_rem >= len {
                self.a.copy_to_bytes(len)
            } else if a_rem == 0 {
                self.b.copy_to_bytes(len)
            } else {
                assert!(
                    len - a_rem <= self.b.remaining(),
                    "`len` greater than remaining"
                );
                crate::Bytes::new()
            }
        }
    }
    
    let mut chain = MockChain {
        a: MockA { remaining: 2 },
        b: MockB { remaining: 1 },
    };
    
    chain.copy_to_bytes(4);
}

