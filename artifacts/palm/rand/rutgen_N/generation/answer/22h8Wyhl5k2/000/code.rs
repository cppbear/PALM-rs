// Answer 0

#[test]
fn test_advance_positive_delta() {
    struct Pcg {
        state: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus: u128 = 0;
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

    let mut pcg = Pcg { state: 1 };
    pcg.advance(1);
    assert_eq!(pcg.state, 6364136223846793006);
}

#[test]
fn test_advance_zero_delta() {
    struct Pcg {
        state: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus: u128 = 0;
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

    let mut pcg = Pcg { state: 42 };
    pcg.advance(0);
    assert_eq!(pcg.state, 42);
}

#[test]
fn test_advance_large_delta() {
    struct Pcg {
        state: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl Pcg {
        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
            let mut cur_plus: u128 = 0;
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

    let mut pcg = Pcg { state: 1 };
    pcg.advance(128);
    assert!(pcg.state > 1);
}

