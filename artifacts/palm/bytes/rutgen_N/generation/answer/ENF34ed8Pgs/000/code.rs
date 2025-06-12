// Answer 0

#[test]
fn test_advance_with_only_a_remaining() {
    struct TestBuf {
        remaining: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf { remaining: size }
        }

        fn remaining(&self) -> usize {
            self.remaining
        }

        fn advance(&mut self, cnt: usize) {
            if cnt >= self.remaining {
                self.remaining = 0;
            } else {
                self.remaining -= cnt;
            }
        }
    }

    struct Chain {
        a: TestBuf,
        b: TestBuf,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: TestBuf::new(a_size),
                b: TestBuf::new(b_size),
            }
        }

        fn advance(&mut self, cnt: usize) {
            let a_rem = self.a.remaining();

            if a_rem != 0 {
                if a_rem >= cnt {
                    self.a.advance(cnt);
                    return;
                }

                self.a.advance(a_rem);
            }

            self.b.advance(cnt - self.a.remaining());
        }
    }

    let mut chain = Chain::new(5, 10);
    chain.advance(3);
    assert_eq!(chain.a.remaining(), 2);
    assert_eq!(chain.b.remaining(), 10);
}

#[test]
fn test_advance_with_a_exhausted() {
    struct TestBuf {
        remaining: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf { remaining: size }
        }

        fn remaining(&self) -> usize {
            self.remaining
        }

        fn advance(&mut self, cnt: usize) {
            if cnt >= self.remaining {
                self.remaining = 0;
            } else {
                self.remaining -= cnt;
            }
        }
    }

    struct Chain {
        a: TestBuf,
        b: TestBuf,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: TestBuf::new(a_size),
                b: TestBuf::new(b_size),
            }
        }

        fn advance(&mut self, cnt: usize) {
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

    let mut chain = Chain::new(5, 10);
    chain.advance(5);
    assert_eq!(chain.a.remaining(), 0);
    assert_eq!(chain.b.remaining(), 10);
}

#[test]
fn test_advance_with_b_only() {
    struct TestBuf {
        remaining: usize,
    }

    impl TestBuf {
        fn new(size: usize) -> Self {
            TestBuf { remaining: size }
        }

        fn remaining(&self) -> usize {
            self.remaining
        }

        fn advance(&mut self, cnt: usize) {
            if cnt >= self.remaining {
                self.remaining = 0;
            } else {
                self.remaining -= cnt;
            }
        }
    }

    struct Chain {
        a: TestBuf,
        b: TestBuf,
    }

    impl Chain {
        fn new(a_size: usize, b_size: usize) -> Self {
            Chain {
                a: TestBuf::new(a_size),
                b: TestBuf::new(b_size),
            }
        }

        fn advance(&mut self, cnt: usize) {
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

    let mut chain = Chain::new(0, 10);
    chain.advance(5);
    assert_eq!(chain.a.remaining(), 0);
    assert_eq!(chain.b.remaining(), 5);
}

