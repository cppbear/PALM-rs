// Answer 0

#[test]
fn test_get_mut_with_valid_inner() {
    struct SimpleBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl SimpleBuf {
        pub fn new(data: Vec<u8>) -> Self {
            SimpleBuf { data, position: 0 }
        }

        pub fn advance(&mut self, count: usize) {
            self.position = cmp::min(self.position + count, self.data.len());
        }

        pub fn remaining(&self) -> usize {
            self.data.len() - self.position
        }
    }

    let mut inner = SimpleBuf::new(b"hello world".to_vec());
    let mut take = Take { inner, limit: 10 };

    let inner_mut = take.get_mut();
    assert_eq!(inner_mut.remaining(), 11); // before advancing

    inner_mut.advance(2);
    assert_eq!(inner_mut.remaining(), 9); // after advancing
}

#[test]
fn test_get_mut_with_inherent_mutability() {
    struct MutableBuf {
        value: usize,
    }

    impl MutableBuf {
        pub fn new(value: usize) -> Self {
            MutableBuf { value }
        }

        pub fn increment(&mut self) {
            self.value += 1;
        }
    }

    let mut inner = MutableBuf::new(5);
    let mut take = Take { inner, limit: 20 };

    {
        let inner_mut = take.get_mut();
        inner_mut.increment();
        assert_eq!(inner_mut.value, 6);
    }

    {
        let inner_mut = take.get_mut();
        inner_mut.increment();
        assert_eq!(inner_mut.value, 7);
    }
}

#[test]
#[should_panic]
fn test_get_mut_with_panic_due_to_mutability() {
    struct PanicBuf {
        data: [u8; 3],
        position: usize,
    }

    impl PanicBuf {
        pub fn new(data: [u8; 3]) -> Self {
            PanicBuf { data, position: 0 }
        }

        pub fn advance(&mut self, count: usize) {
            self.position += count; // no bounds checking for this test case
        }
    }

    let mut inner = PanicBuf::new([1, 2, 3]);
    let mut take = Take { inner, limit: 5 };

    let inner_mut = take.get_mut();
    inner_mut.advance(5); // will induce panic in a real world scenario since it may exceed bounds
}

