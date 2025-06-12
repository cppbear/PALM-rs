// Answer 0

#[test]
fn test_next_u64_full_range() {
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

    let mut rng = Xoshiro256PlusPlus::new([1, 2, 3, 4]);
    let result = rng.next_u64();
    assert!(result != 0, "The output should be non-zero");

    let mut rng_zero = Xoshiro256PlusPlus::new([0, 0, 0, 0]);
    let result_zero = rng_zero.next_u64();
    assert!(result_zero != 0, "The output should be non-zero even from zero seed");
}

#[test]
fn test_next_u64_negative_cases() {
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
    assert!(result != u64::MAX, "The output should not be the maximum u64 value");
}

