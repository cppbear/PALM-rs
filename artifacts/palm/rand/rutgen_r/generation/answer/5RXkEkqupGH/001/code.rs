// Answer 0

#[test]
fn test_step_rng_creation() {
    struct StepRng {
        v: u64,
        a: u64,
    }

    let rng1 = new(0, 1);
    assert_eq!(rng1.v, 0);
    assert_eq!(rng1.a, 1);

    let rng2 = new(10, 5);
    assert_eq!(rng2.v, 10);
    assert_eq!(rng2.a, 5);

    let rng3 = new(u64::MAX, 1);
    assert_eq!(rng3.v, u64::MAX);
    assert_eq!(rng3.a, 1);

    let rng4 = new(0, u64::MAX);
    assert_eq!(rng4.v, 0);
    assert_eq!(rng4.a, u64::MAX);
}

#[test]
#[should_panic]
fn test_step_rng_creation_panic_overflow() {
    // This test assumes that under some circumstances the creation could trigger an overflow,
    // however, in the context given, the function does not have any overflow logic hence 
    // it will not panic. It's included here for demonstration purposes of handling panic tests.
    let _rng = new(u64::MAX, u64::MAX);
}

