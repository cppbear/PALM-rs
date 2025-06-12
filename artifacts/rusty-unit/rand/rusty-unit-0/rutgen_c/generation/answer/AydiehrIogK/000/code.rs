// Answer 0

#[test]
fn test_next_u32() {
    struct Xoshiro256PlusPlusTest {
        s: [u64; 4],
    }

    impl RngCore for Xoshiro256PlusPlusTest {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            let val = self.next_u64();
            (val >> 32) as u32
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            let res = self
                .s[0]
                .wrapping_add(self.s[3])
                .rotate_left(23)
                .wrapping_add(self.s[0]);
            let t = self.s[1] << 17;
            self.s[2] ^= self.s[0];
            self.s[3] ^= self.s[1];
            self.s[1] ^= self.s[2];
            self.s[0] ^= self.s[3];
            self.s[2] ^= t;
            self.s[3] = self.s[3].rotate_left(45);
            res
        }

        #[inline]
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = Xoshiro256PlusPlusTest { s: [0, 0, 0, 0] };

    let output1 = rng.next_u32();
    let output2 = rng.next_u32();

    assert!(output1 != output2);
}

#[test]
fn test_next_u32_boundary_values() {
    struct Xoshiro256PlusPlusTest {
        s: [u64; 4],
    }

    impl RngCore for Xoshiro256PlusPlusTest {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            let val = self.next_u64();
            (val >> 32) as u32
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            let res = self
                .s[0]
                .wrapping_add(self.s[3])
                .rotate_left(23)
                .wrapping_add(self.s[0]);
            let t = self.s[1] << 17;
            self.s[2] ^= self.s[0];
            self.s[3] ^= self.s[1];
            self.s[1] ^= self.s[2];
            self.s[0] ^= self.s[3];
            self.s[2] ^= t;
            self.s[3] = self.s[3].rotate_left(45);
            res
        }

        #[inline]
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    let mut rng = Xoshiro256PlusPlusTest { s: [u64::MAX, u64::MAX, u64::MAX, u64::MAX] };

    let output = rng.next_u32();

    assert!(output <= u32::MAX);
}

