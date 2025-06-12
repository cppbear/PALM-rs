// Answer 0

#[test]
fn test_advance_a_rem_not_zero_and_a_rem_equals_cnt() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>) -> Self {
            Buffer { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain {
        a: Buffer,
        b: Buffer,
    }

    impl Chain {
        fn new(a: Buffer, b: Buffer) -> Self {
            Chain { a, b }
        }

        fn advance(&mut self, mut cnt: usize) {
            let a_rem = self.a.remaining();

            if a_rem != 0 {
                if a_rem >= cnt {
                    self.a.advance(cnt);
                    return;
                }

                self.a.advance(a_rem);
                cnt -= a_rem;
            }

            self.b.advance(cnt);
        }
    }

    let buf_a = Buffer::new(vec![0, 1, 2, 3]);
    let buf_b = Buffer::new(vec![4, 5, 6, 7]);

    let mut chain = Chain::new(buf_a, buf_b);
    
    assert_eq!(chain.a.remaining(), 4);
    assert_eq!(chain.b.remaining(), 4);

    // Test to "advance" exactly the remaining bytes in buffer a
    chain.advance(4);

    assert_eq!(chain.a.remaining(), 0);
    assert_eq!(chain.b.remaining(), 4);
}

#[test]
fn test_advance_a_rem_not_zero_and_a_rem_greater_than_cnt() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>) -> Self {
            Buffer { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain {
        a: Buffer,
        b: Buffer,
    }

    impl Chain {
        fn new(a: Buffer, b: Buffer) -> Self {
            Chain { a, b }
        }

        fn advance(&mut self, mut cnt: usize) {
            let a_rem = self.a.remaining();

            if a_rem != 0 {
                if a_rem >= cnt {
                    self.a.advance(cnt);
                    return;
                }

                self.a.advance(a_rem);
                cnt -= a_rem;
            }

            self.b.advance(cnt);
        }
    }

    let buf_a = Buffer::new(vec![0, 1, 2, 3, 4]);
    let buf_b = Buffer::new(vec![5, 6, 7, 8]);

    let mut chain = Chain::new(buf_a, buf_b);
    
    assert_eq!(chain.a.remaining(), 5);
    assert_eq!(chain.b.remaining(), 4);

    // Test to "advance" less than the remaining bytes in buffer a
    chain.advance(3);

    assert_eq!(chain.a.remaining(), 2);
    assert_eq!(chain.b.remaining(), 4);
}

