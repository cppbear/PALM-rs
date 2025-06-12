// Answer 0

#[test]
fn test_from_state_incr() {
    struct Lcg128Xsl64 {
        state: u128,
        increment: u128,
    }

    impl Lcg128Xsl64 {
        fn step(&mut self) {
            // Simulate the step function logic (example placeholder)
            self.state = self.state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
        }
    }

    let state = 0x123456789ABCDEF0;
    let increment = 0x01;
    let mut pcg = Lcg128Xsl64 { state, increment };

    // Move away from initial value
    pcg.state = pcg.state.wrapping_add(pcg.increment);
    pcg.step();

    assert!(pcg.state != state);
}

#[test]
fn test_from_state_incr_boundary() {
    struct Lcg128Xsl64 {
        state: u128,
        increment: u128,
    }

    impl Lcg128Xsl64 {
        fn step(&mut self) {
            self.state = self.state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
        }
    }

    let state = u128::MAX;
    let increment = u128::MAX;
    let mut pcg = Lcg128Xsl64 { state, increment };

    pcg.state = pcg.state.wrapping_add(pcg.increment);
    pcg.step();

    assert!(pcg.state != state);
}

