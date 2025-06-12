// Answer 0

#[derive(Debug)]
struct Pcg64 {
    state: u64,
    increment: u64,
}

const MULTIPLIER: u64 = 6364136223846793005;

impl Pcg64 {
    fn new(state: u64, increment: u64) -> Self {
        Pcg64 { state, increment }
    }

    pub fn advance(&mut self, delta: u64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
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

#[test]
fn test_advance_with_positive_delta() {
    let mut rng = Pcg64::new(42, 54);
    rng.advance(1);
    assert!(rng.state != 42); // Ensure the state has been changed
}

#[test]
fn test_advance_with_odd_delta() {
    let mut rng = Pcg64::new(42, 54);
    rng.advance(3);
    assert!(rng.state != 42); // Ensure the state is different after advancing
}

#[should_panic]
#[test]
fn test_advance_with_zero_delta() {
    let mut rng = Pcg64::new(42, 54);
    rng.advance(0); // This should not panic as per the function's constraints but is a boundary case
}

#[test]
fn test_advance_with_large_odd_delta() {
    let mut rng = Pcg64::new(42, 54);
    rng.advance(u64::MAX); // Testing with a large delta, ensuring it doesn't panic
    assert!(rng.state != 42); // Ensure the state has been changed
}

