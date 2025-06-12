// Answer 0

#[derive(Debug)]
struct Pcg128cm {
    state: u128,
    increment: u128,
}

const MULTIPLIER: u64 = 6364136223846793005;

impl Pcg128cm {
    fn new(state: u128, increment: u128) -> Self {
        Pcg128cm { state, increment }
    }

    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = MULTIPLIER as u128;
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

#[test]
fn test_advance_positive_delta() {
    let mut pcg = Pcg128cm::new(0x123456789abcdef0, 0x1);
    pcg.advance(1);
    assert!(pcg.state != 0x123456789abcdef0); // Ensure state has changed
}

#[test]
fn test_advance_large_delta() {
    let mut pcg = Pcg128cm::new(0x123456789abcdef0, 0x1);
    pcg.advance(1000);
    assert!(pcg.state != 0x123456789abcdef0); // Ensure state has changed
}

#[test]
fn test_advance_odd_delta() {
    let mut pcg = Pcg128cm::new(0x123456789abcdef0, 0x1);
    pcg.advance(3);
    assert!(pcg.state != 0x123456789abcdef0); // Ensure state has changed
}

#[should_panic]
#[test]
fn test_advance_zero_delta() {
    let mut pcg = Pcg128cm::new(0x123456789abcdef0, 0x1);
    pcg.advance(0); // This should not panic but shouldn't change the state either.
    assert_eq!(pcg.state, 0x123456789abcdef0);
}

