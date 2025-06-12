// Answer 0

#[test]
fn test_log2_pow5_zero() {
    assert_eq!(ryu::log2_pow5(0), 0);
}

#[test]
fn test_log2_pow5_small() {
    assert_eq!(ryu::log2_pow5(1), 1);
    assert_eq!(ryu::log2_pow5(2), 2);
}

#[test]
fn test_log2_pow5_large() {
    assert_eq!(ryu::log2_pow5(3528), 3992);
}

#[test]
#[should_panic]
fn test_log2_pow5_negative() {
    let _ = ryu::log2_pow5(-1);
}

#[test]
#[should_panic]
fn test_log2_pow5_too_large() {
    let _ = ryu::log2_pow5(3529);
}

