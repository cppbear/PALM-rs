// Answer 0

#[test]
fn test_advance_mut_a_rem_greater_than_zero_and_cnt_greater_than_a_rem() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn new(size: usize) -> Self {
            Buffer {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain {
        a: Buffer,
        b: Buffer,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: Buffer::new(a_size),
                b: Buffer::new(b_size),
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

    let mut chain = Chain::new(5, 10);

    unsafe {
        chain.advance_mut(10); // This should consume all of a and then proceed to b
    }

    assert_eq!(chain.a.remaining_mut(), 0);
    assert_eq!(chain.b.position, 5); // Since a had 5 and it was entirely consumed
}

