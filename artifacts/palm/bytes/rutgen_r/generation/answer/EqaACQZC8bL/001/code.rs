// Answer 0

#[test]
fn test_advance_mut_with_a_rem_eq_cnt() {
    struct MockBuffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockBuffer {
        fn new(size: usize) -> Self {
            MockBuffer {
                data: vec![0; size],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    struct Chain {
        a: MockBuffer,
        b: MockBuffer,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: MockBuffer::new(a_size),
                b: MockBuffer::new(b_size),
            }
        }

        unsafe fn advance_mut(&mut self, mut cnt: usize) {
            let a_rem = self.a.remaining_mut();

            if a_rem != 0 {
                if a_rem >= cnt {
                    self.a.advance_mut(cnt);
                    return;
                }

                self.a.advance_mut(a_rem);
                cnt -= a_rem;
            }

            self.b.advance_mut(cnt);
        }
    }

    let mut chain = Chain::new(5, 3); // a has 5 remaining units, b has 3
    let mut cnt = 5; // This equals to a_rem

    unsafe {
        chain.advance_mut(cnt);
    }

    assert_eq!(chain.a.pos, 5); // a should have advanced by 5
    assert_eq!(chain.b.pos, 0); // b should not have advanced
}

#[test]
fn test_advance_mut_with_a_rem_eq_cnt_and_boundary() {
    struct MockBuffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockBuffer {
        fn new(size: usize) -> Self {
            MockBuffer {
                data: vec![0; size],
                pos: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.pos
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.pos += cnt;
        }
    }

    struct Chain {
        a: MockBuffer,
        b: MockBuffer,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: MockBuffer::new(a_size),
                b: MockBuffer::new(b_size),
            }
        }

        unsafe fn advance_mut(&mut self, mut cnt: usize) {
            let a_rem = self.a.remaining_mut();

            if a_rem != 0 {
                if a_rem >= cnt {
                    self.a.advance_mut(cnt);
                    return;
                }

                self.a.advance_mut(a_rem);
                cnt -= a_rem;
            }

            self.b.advance_mut(cnt);
        }
    }

    let mut chain = Chain::new(3, 0); // a has 3 remaining units, b has 0
    let cnt = 3; // This equals to a_rem

    unsafe {
        chain.advance_mut(cnt);
    }

    assert_eq!(chain.a.pos, 3); // a should have advanced by 3
    assert_eq!(chain.b.pos, 0); // b should not have advanced
}

