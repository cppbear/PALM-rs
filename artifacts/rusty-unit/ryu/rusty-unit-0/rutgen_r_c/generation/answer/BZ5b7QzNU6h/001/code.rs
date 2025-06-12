// Answer 0

#[test]
fn test_ceil_log2_pow5() {
    // Testing the minimum valid input
    assert_eq!(ceil_log2_pow5(0), 1);

    // Testing a mid-range input
    assert_eq!(ceil_log2_pow5(1000), log2_pow5(1000) + 1);
    
    // Testing the boundary case of e = 3528 (max allowed input)
    assert_eq!(ceil_log2_pow5(3528), log2_pow5(3528) + 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_ceil_log2_pow5_negative() {
    // Testing invalid input (negative value should panic)
    ceil_log2_pow5(-1);
}

