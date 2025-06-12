// Answer 0

#[test]
fn test_step_with_positive_state() {
    struct Pcg {
        state: u128,
        increment: u128,
    }
    
    let mut pcg = Pcg {
        state: 1,
        increment: 2,
    };
    
    pcg.step();
    assert_eq!(pcg.state, 1.wrapping_mul(MULTIPLIER as u128).wrapping_add(2));
}

#[test]
fn test_step_with_zero_increment() {
    struct Pcg {
        state: u128,
        increment: u128,
    }
    
    let mut pcg = Pcg {
        state: 1000,
        increment: 0,
    };

    pcg.step();
    assert_eq!(pcg.state, 1000.wrapping_mul(MULTIPLIER as u128).wrapping_add(0));
}

#[test]
fn test_step_with_max_u128_state() {
    struct Pcg {
        state: u128,
        increment: u128,
    }
    
    let mut pcg = Pcg {
        state: u128::MAX,
        increment: 1,
    };

    pcg.step();
    assert_eq!(pcg.state, u128::MAX.wrapping_mul(MULTIPLIER as u128).wrapping_add(1));
}

#[should_panic]
fn test_step_with_overflow_increment() {
    struct Pcg {
        state: u128,
        increment: u128,
    }
    
    let mut pcg = Pcg {
        state: 10,
        increment: u128::MAX,
    };

    pcg.step(); // This test is expected to panic due to overflow in the wrapping_add operation.
}

