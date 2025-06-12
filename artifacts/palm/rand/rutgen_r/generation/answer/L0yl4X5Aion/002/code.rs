// Answer 0

#[test]
fn test_advance_positive_mdelta() {
    struct Pcg {
        state: u128,
        increment: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn new(state: u128, increment: u128) -> Self {
            Pcg { state, increment }
        }

        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus = self.increment;
            let mut mdelta = delta;

            while mdelta > 0 {
                if (mdelta & 1) != 0 {
                    acc_mult = acc_mult.wrapping_mul(cur_mult);
                    acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
                }
                cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
                cur_mult = cur_mult.wrapping_mul(cur_mult);
                mdelta /= 2;
            }
            self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
        }
    }

    let mut rng = Pcg::new(1, 1);
    rng.advance(10);
    assert!(rng.state > 0); // Just an assertion that state advances
}

#[test]
fn test_advance_zero_mdelta() {
    struct Pcg {
        state: u128,
        increment: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn new(state: u128, increment: u128) -> Self {
            Pcg { state, increment }
        }

        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus = self.increment;
            let mut mdelta = delta;

            while mdelta > 0 {
                if (mdelta & 1) != 0 {
                    acc_mult = acc_mult.wrapping_mul(cur_mult);
                    acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
                }
                cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
                cur_mult = cur_mult.wrapping_mul(cur_mult);
                mdelta /= 2;
            }
            self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
        }
    }

    let mut rng = Pcg::new(5, 1);
    rng.advance(0);
    assert_eq!(rng.state, 5); // State should remain unchanged
}

#[test]
#[should_panic]
fn test_advance_negative_mdelta() {
    struct Pcg {
        state: u128,
        increment: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn new(state: u128, increment: u128) -> Self {
            Pcg { state, increment }
        }

        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus = self.increment;
            let mut mdelta = delta;

            while mdelta > 0 {
                if (mdelta & 1) != 0 {
                    acc_mult = acc_mult.wrapping_mul(cur_mult);
                    acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
                }
                cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
                cur_mult = cur_mult.wrapping_mul(cur_mult);
                mdelta /= 2;
            }
            self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
        }
    }

    let mut rng = Pcg::new(1, 1);
    rng.advance(u128::MAX); // This can create a panic depending on internal state
}

