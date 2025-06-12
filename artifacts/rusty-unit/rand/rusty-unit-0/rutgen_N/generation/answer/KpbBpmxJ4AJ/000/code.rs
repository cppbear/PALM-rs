// Answer 0

#[test]
fn test_next_u64() {
    struct Xoshiro256PlusPlus {
        s: [u64; 4],
    }

    impl Xoshiro256PlusPlus {
        fn new(seed: [u64; 4]) -> Self {
            Xoshiro256PlusPlus { s: seed }
        }

        fn next_u64(&mut self) -> u64 {
            let res = self.s[0]
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
    }

    let mut rng = Xoshiro256PlusPlus::new([42, 54, 32, 7]);
    let result = rng.next_u64();
    assert!(result != 0); // Ensure that the result is not zero

    let result2 = rng.next_u64();
    assert!(result2 != result); // Ensure that subsequent results are different
}

#[test]
fn test_next_u64_boundary() {
    struct Xoshiro256PlusPlus {
        s: [u64; 4],
    }

    impl Xoshiro256PlusPlus {
        fn new(seed: [u64; 4]) -> Self {
            Xoshiro256PlusPlus { s: seed }
        }

        fn next_u64(&mut self) -> u64 {
            let res = self.s[0]
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
    }

    let mut rng = Xoshiro256PlusPlus::new([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
    let result = rng.next_u64();
    assert!(result != 0); // Ensure that the result is not zero
    
    // since we're at the max value, the wrapping addition might give something interesting
    let result2 = rng.next_u64();
    assert!(result2 != result); // Ensure that subsequent results are different
}

