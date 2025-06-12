// Answer 0

#[test]
fn test_from_state_incr() {
    struct TestStruct {
        state: u64,
        increment: u64,
    }
    
    impl TestStruct {
        fn from_state_incr(state: u64, increment: u64) -> Lcg64Xsh32 {
            let mut pcg = Lcg64Xsh32 { state, increment };
            pcg.state = pcg.state.wrapping_add(pcg.increment);
            pcg.step();
            pcg
        }
    }

    let rng1 = TestStruct::from_state_incr(0, 1);
    assert_eq!(rng1.state, 6364136223846793006); // MULTIPLIER * 0 + 1

    let rng2 = TestStruct::from_state_incr(1, 2);
    assert_eq!(rng2.state, 12728272447679694967); // MULTIPLIER * 1 + 2

    let rng3 = TestStruct::from_state_incr(u64::MAX, u64::MAX);
    assert_eq!(rng3.state, 16439066386801235829); // Multiplication and wrapping on max values
}

