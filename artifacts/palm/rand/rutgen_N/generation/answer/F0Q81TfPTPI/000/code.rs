// Answer 0

#[test]
fn test_flip_c_heads_true() {
    struct TestRng {
        chunk: u32,
        chunk_remaining: u32,
    }

    impl TestRng {
        fn new() -> Self {
            Self {
                chunk: 0b00000000_00000000_00000000_00001111, // Simulating heads
                chunk_remaining: 32,
            }
        }

        fn next_u32(&mut self) -> u32 {
            0 // No new bits are generated for this test
        }

        fn flip_c_heads(&mut self, c: u32) -> bool {
            debug_assert!(c <= 32);
            loop {
                let zeros = self.chunk.leading_zeros();
                if zeros < c {
                    self.chunk = self.chunk.wrapping_shl(zeros + 1);
                    self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                    return false;
                } else {
                    if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                        self.chunk_remaining = new_remaining;
                        self.chunk <<= c;
                        return true;
                    } else {
                        c -= self.chunk_remaining;
                        self.chunk = self.next_u32();
                        self.chunk_remaining = 32;
                    }
                }
            }
        }
    }

    let mut rng = TestRng::new();
    assert_eq!(rng.flip_c_heads(4), true);
}

#[test]
fn test_flip_c_heads_false() {
    struct TestRng {
        chunk: u32,
        chunk_remaining: u32,
    }

    impl TestRng {
        fn new() -> Self {
            Self {
                chunk: 0b11111111_11111111_11111111_11110000, // Simulating tails after heads
                chunk_remaining: 32,
            }
        }

        fn next_u32(&mut self) -> u32 {
            0 // No new bits are generated for this test
        }

        fn flip_c_heads(&mut self, c: u32) -> bool {
            debug_assert!(c <= 32);
            loop {
                let zeros = self.chunk.leading_zeros();
                if zeros < c {
                    self.chunk = self.chunk.wrapping_shl(zeros + 1);
                    self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                    return false;
                } else {
                    if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                        self.chunk_remaining = new_remaining;
                        self.chunk <<= c;
                        return true;
                    } else {
                        c -= self.chunk_remaining;
                        self.chunk = self.next_u32();
                        self.chunk_remaining = 32;
                    }
                }
            }
        }
    }

    let mut rng = TestRng::new();
    assert_eq!(rng.flip_c_heads(4), false);
}

#[test]
fn test_flip_c_heads_boundary_zero() {
    struct TestRng {
        chunk: u32,
        chunk_remaining: u32,
    }

    impl TestRng {
        fn new() -> Self {
            Self {
                chunk: 0b00000000_00000000_00000000_00000000, // All heads
                chunk_remaining: 32,
            }
        }

        fn next_u32(&mut self) -> u32 {
            0 // No new bits are generated for this test
        }

        fn flip_c_heads(&mut self, c: u32) -> bool {
            debug_assert!(c <= 32);
            loop {
                let zeros = self.chunk.leading_zeros();
                if zeros < c {
                    self.chunk = self.chunk.wrapping_shl(zeros + 1);
                    self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                    return false;
                } else {
                    if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                        self.chunk_remaining = new_remaining;
                        self.chunk <<= c;
                        return true;
                    } else {
                        c -= self.chunk_remaining;
                        self.chunk = self.next_u32();
                        self.chunk_remaining = 32;
                    }
                }
            }
        }
    }

    let mut rng = TestRng::new();
    assert_eq!(rng.flip_c_heads(0), true);
}

#[test]
fn test_flip_c_heads_boundary_max() {
    struct TestRng {
        chunk: u32,
        chunk_remaining: u32,
    }

    impl TestRng {
        fn new() -> Self {
            Self {
                chunk: 0b00000000_00000000_00000000_00001111, // Simulating heads
                chunk_remaining: 32,
            }
        }

        fn next_u32(&mut self) -> u32 {
            0 // No new bits are generated for this test
        }

        fn flip_c_heads(&mut self, c: u32) -> bool {
            debug_assert!(c <= 32);
            loop {
                let zeros = self.chunk.leading_zeros();
                if zeros < c {
                    self.chunk = self.chunk.wrapping_shl(zeros + 1);
                    self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                    return false;
                } else {
                    if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                        self.chunk_remaining = new_remaining;
                        self.chunk <<= c;
                        return true;
                    } else {
                        c -= self.chunk_remaining;
                        self.chunk = self.next_u32();
                        self.chunk_remaining = 32;
                    }
                }
            }
        }
    }

    let mut rng = TestRng::new();
    assert_eq!(rng.flip_c_heads(32), true);
}

