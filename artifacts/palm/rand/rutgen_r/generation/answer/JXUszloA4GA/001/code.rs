// Answer 0

#[test]
fn test_next_u64_basic() {
    struct MockRng {
        v: u64,
        a: u64,
    }

    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
    }

    let mut rng = MockRng { v: 1, a: 2 };
    assert_eq!(rng.next_u64(), 1);
    assert_eq!(rng.next_u64(), 3);
    assert_eq!(rng.next_u64(), 5);
}

#[test]
fn test_next_u64_with_wrapping() {
    struct MockRng {
        v: u64,
        a: u64,
    }

    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
    }

    let mut rng = MockRng { v: u64::MAX - 1, a: 2 };
    assert_eq!(rng.next_u64(), u64::MAX - 1);
    assert_eq!(rng.next_u64(), u64::MAX + 1 - 1); // Wraps to 0
    assert_eq!(rng.next_u64(), 1);
}

#[test]
fn test_next_u64_with_zero_increment() {
    struct MockRng {
        v: u64,
        a: u64,
    }

    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
    }

    let mut rng = MockRng { v: 5, a: 0 };
    assert_eq!(rng.next_u64(), 5);
    assert_eq!(rng.next_u64(), 5); // Should stay the same
    assert_eq!(rng.next_u64(), 5); // Should stay the same
}

#[test]
fn test_next_u64_large_increment() {
    struct MockRng {
        v: u64,
        a: u64,
    }

    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
    }

    let mut rng = MockRng { v: 1, a: u64::MAX };
    assert_eq!(rng.next_u64(), 1);
    assert_eq!(rng.next_u64(), 0); // Wraps around
}

