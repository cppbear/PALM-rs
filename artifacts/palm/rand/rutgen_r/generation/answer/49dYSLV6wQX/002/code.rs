// Answer 0

#[derive(Default)]
struct Pcg128cm {
    state: u128,
    increment: u128,
}

const MULTIPLIER: u64 = 6364136223846793005;

impl Pcg128cm {
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
fn test_advance_positive_mdelta() {
    let mut rng = Pcg128cm {
        state: 1,
        increment: 1,
    };
    let delta: u128 = 3; // mdelta > 0 and (mdelta & 1) != 0 at first iteration.
    rng.advance(delta);
    assert!(rng.state > 0);
}

#[test]
fn test_advance_even_mdelta() {
    let mut rng = Pcg128cm {
        state: 1,
        increment: 1,
    };
    let delta: u128 = 2; // mdelta > 0 and (mdelta & 1) == 0 at first iteration.
    rng.advance(delta);
    assert!(rng.state > 0);
}

#[test]
fn test_advance_zero_mdelta() {
    let mut rng = Pcg128cm {
        state: 1,
        increment: 1,
    };
    let delta: u128 = 0; // mdelta == 0, should not panic.
    rng.advance(delta);
    assert_eq!(rng.state, 1); // State should remain unchanged.
}

