// Answer 0

#[test]
fn test_decoded_len_estimate() {
    assert_eq!(3, decoded_len_estimate(1));
    assert_eq!(3, decoded_len_estimate(2));
    assert_eq!(3, decoded_len_estimate(3));
    assert_eq!(3, decoded_len_estimate(4));
    assert_eq!(6, decoded_len_estimate(5));
}

#[test]
fn test_decoded_len_estimate_boundary() {
    assert_eq!(0, decoded_len_estimate(0)); // edge case for zero length
    assert_eq!(3, decoded_len_estimate(3)); // boundary of one full group
    assert_eq!(3, decoded_len_estimate(6)); // boundary of two full groups
    assert_eq!(9, decoded_len_estimate(7)); // Just over two full groups
}

