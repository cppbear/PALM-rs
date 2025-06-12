// Answer 0

#[test]
fn test_gen_mod_u64_with_lo_equal_n() {
    struct TestRng {
        state: u64,
    }

    impl TestRng {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn gen_u64(&mut self) -> u64 {
            const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
            const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;
            let s = self.state.wrapping_add(WY_CONST_0);
            self.state = s;
            let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
            (t as u64) ^ (t >> 64) as u64
        }

        fn gen_mod_u64(&mut self, n: u64) -> u64 {
            let mut r = self.gen_u64();
            let mut hi = mul_high_u64(r, n);
            let mut lo = r.wrapping_mul(n);
            if lo < n {
                let t = n.wrapping_neg() % n;
                while lo < t {
                    r = self.gen_u64();
                    hi = mul_high_u64(r, n);
                    lo = r.wrapping_mul(n);
                }
            }
            hi
        }
    }

    let mut rng = TestRng::new(42);
    let n = 42; // Setting n to a value
    let hi = rng.gen_mod_u64(n);
    assert!(hi < n); // Expecting a proper high value less than n
}

#[test]
#[should_panic(expected = "empty range")]
fn test_gen_mod_u64_with_empty_range() {
    struct TestRng {
        state: u64,
    }

    impl TestRng {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn gen_u64(&mut self) -> u64 {
            const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
            const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;
            let s = self.state.wrapping_add(WY_CONST_0);
            self.state = s;
            let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
            (t as u64) ^ (t >> 64) as u64
        }

        fn gen_mod_u64(&mut self, n: u64) -> u64 {
            let mut r = self.gen_u64();
            let mut hi = mul_high_u64(r, n);
            let mut lo = r.wrapping_mul(n);
            if lo < n {
                let t = n.wrapping_neg() % n;
                while lo < t {
                    r = self.gen_u64();
                    hi = mul_high_u64(r, n);
                    lo = r.wrapping_mul(n);
                }
            }
            hi
        }
    }

    let mut rng = TestRng::new(0);
    let n = 0; // Setting to 0, which should trigger a panic due to empty range
    rng.gen_mod_u64(n);
}

#[test]
fn test_gen_mod_u64_lower_bound() {
    struct TestRng {
        state: u64,
    }

    impl TestRng {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn gen_u64(&mut self) -> u64 {
            const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
            const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;
            let s = self.state.wrapping_add(WY_CONST_0);
            self.state = s;
            let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
            (t as u64) ^ (t >> 64) as u64
        }

        fn gen_mod_u64(&mut self, n: u64) -> u64 {
            let mut r = self.gen_u64();
            let mut hi = mul_high_u64(r, n);
            let mut lo = r.wrapping_mul(n);
            if lo < n {
                let t = n.wrapping_neg() % n;
                while lo < t {
                    r = self.gen_u64();
                    hi = mul_high_u64(r, n);
                    lo = r.wrapping_mul(n);
                }
            }
            hi
        }
    }

    let mut rng = TestRng::new(1);
    let n = 1; // Setting n to 1
    let hi = rng.gen_mod_u64(n);
    assert!(hi < n); // Expecting a proper high value less than n
}

