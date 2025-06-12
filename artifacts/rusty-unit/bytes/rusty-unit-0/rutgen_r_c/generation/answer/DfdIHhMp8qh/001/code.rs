// Answer 0

#[test]
fn test_first_mut() {
    struct BufImpl {
        data: Vec<u8>,
    }

    impl BufMut for BufImpl {
        fn advance(&mut self, cnt: usize) {
            self.data.drain(..cnt);
        }

        fn remaining_mut(&self) -> usize {
            self.data.len()
        }
    }

    let mut buf_a = BufImpl { data: b"hello".to_vec() };
    let buf_b = BufImpl { data: b"world".to_vec() };
    
    let mut chain = Chain::new(buf_a, buf_b);
    
    // Test mutable reference access and modification
    chain.first_mut().advance(1);
    assert_eq!(chain.first_ref().remaining_mut(), 4); // Check remaining in first buffer after advancing
}

#[test]
#[should_panic]
fn test_first_mut_panic() {
    struct BufImpl {
        data: Vec<u8>,
    }

    impl BufMut for BufImpl {
        fn advance(&mut self, cnt: usize) {
            self.data.drain(..cnt);
        }

        fn remaining_mut(&self) -> usize {
            self.data.len()
        }
    }

    let mut buf_a = BufImpl { data: Vec::new() }; // Empty buffer to trigger panic
    let buf_b = BufImpl { data: b"world".to_vec() };
    
    let mut chain = Chain::new(buf_a, buf_b);

    // Attempt to access mutable reference on empty buffer
    chain.first_mut().advance(1); // This should panic
}

