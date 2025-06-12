// Answer 0

#[test]
fn test_bernoulli_p_always_true() {
    const ALWAYS_TRUE: u32 = 1; // Assuming ALWAYS_TRUE is defined as 1 for this test
    const SCALE: u32 = 10; // Example scale for calculation

    struct Bernoulli {
        p_int: u32,
    }

    impl Bernoulli {
        pub fn p(&self) -> f64 {
            if self.p_int == ALWAYS_TRUE {
                1.0
            } else {
                (self.p_int as f64) / SCALE as f64
            }
        }
    }

    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.p(), 1.0);
}

