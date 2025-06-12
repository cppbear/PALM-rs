// Answer 0

#[test]
fn test_lcg128_cm_dxsm64_new() {
    struct Lcg128CmDxsm64 {
        state: u128,
        increment: u128,
    }
    
    impl Lcg128CmDxsm64 {
        pub fn new(state: u128, stream: u128) -> Self {
            let increment = (stream << 1) | 1;
            Self::from_state_incr(state, increment)
        }
        
        fn from_state_incr(state: u128, increment: u128) -> Self {
            let mut pcg = Self { state, increment };
            pcg.state = pcg.state.wrapping_add(pcg.increment);
            pcg.step();
            pcg
        }
        
        fn step(&mut self) {}
    }

    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;

    let pcg = Lcg128CmDxsm64::new(state, stream);
    assert_eq!(pcg.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_lcg128_cm_dxsm64_new_boundary() {
    struct Lcg128CmDxsm64 {
        state: u128,
        increment: u128,
    }
    
    impl Lcg128CmDxsm64 {
        pub fn new(state: u128, stream: u128) -> Self {
            let increment = (stream << 1) | 1;
            Self::from_state_incr(state, increment)
        }
        
        fn from_state_incr(state: u128, increment: u128) -> Self {
            let mut pcg = Self { state, increment };
            pcg.state = pcg.state.wrapping_add(pcg.increment);
            pcg.step();
            pcg
        }
        
        fn step(&mut self) {}
    }

    let state = u128::MAX;
    let stream = u128::MAX;

    let pcg = Lcg128CmDxsm64::new(state, stream);
    assert_eq!(pcg.state, state.wrapping_add((stream << 1) | 1));
}

