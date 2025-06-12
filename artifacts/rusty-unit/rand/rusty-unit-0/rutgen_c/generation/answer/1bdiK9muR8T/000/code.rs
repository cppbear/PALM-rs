// Answer 0

#[test]
fn test_next_u32() {
    struct TestRng {
        v: u64,
        a: u64,
    }

    impl RngCore for TestRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }
        #[inline]
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
        #[inline]
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = TestRng { v: 1, a: 2 };
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 3);
    assert_eq!(rng.next_u32(), 5);
}

#[test]
fn test_next_u32_initial_value_zero() {
    struct TestRng {
        v: u64,
        a: u64,
    }

    impl RngCore for TestRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }
        #[inline]
        fn next_u64(&mut self) -> u64 {
            let res = self.v;
            self.v = self.v.wrapping_add(self.a);
            res
        }
        #[inline]
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = TestRng { v: 0, a: 1 };
    assert_eq!(rng.next_u32(), 0);
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 2);
}

