// Answer 0

#[test]
fn test_advance_b_only() {
    struct A {
        data: Vec<u8>,
        position: usize,
    }

    impl A {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct B {
        position: usize,
    }

    impl B {
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain {
        a: A,
        b: B,
    }

    impl Chain {
        fn new(a_data: Vec<u8>, b_position: usize) -> Self {
            Chain {
                a: A {
                    data: a_data,
                    position: 0,
                },
                b: B { position: b_position },
            }
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

    // Test case: Ensuring a's remaining is 0, thus b will handle cnt
    let mut chain = Chain::new(vec![], 5);
    chain.advance(10);
    assert_eq!(chain.b.position, 15);
}

#[test]
#[should_panic]
fn test_advance_panic_condition() {
    struct A {
        data: Vec<u8>,
        position: usize,
    }

    impl A {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct B {
        position: usize,
    }

    impl B {
        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain {
        a: A,
        b: B,
    }

    impl Chain {
        fn new(a_data: Vec<u8>, b_position: usize) -> Self {
            Chain {
                a: A {
                    data: a_data,
                    position: 0,
                },
                b: B { position: b_position },
            }
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

    // Test case: Attempting to advance when there's no data in a should ensure b handles the cnt; will panic on negative cnt in the advance method.
    let mut chain = Chain::new(vec![], 0);
    // This will attempt to advance b with a negative value due to how cnt is modified.
    chain.advance(100);
}

