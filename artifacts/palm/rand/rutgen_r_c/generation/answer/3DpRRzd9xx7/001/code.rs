// Answer 0

#[test]
fn test_output_xsl_rr_zero_state() {
    let state: u128 = 0;
    let result = output_xsl_rr(state);
    assert_eq!(result, 0);
}

#[test]
fn test_output_xsl_rr_one_state() {
    let state: u128 = 1;
    let result = output_xsl_rr(state);
    assert_eq!(result, 0x8000000000000000);
}

#[test]
fn test_output_xsl_rr_max_state() {
    let state: u128 = u128::MAX;
    let result = output_xsl_rr(state);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_output_xsl_rr_alternating_bits() {
    let state: u128 = 0b1010101010101010101010101010101010101010101010101010101010101010;
    let result = output_xsl_rr(state);
    assert_ne!(result, 0);
}

#[test]
fn test_output_xsl_rr_high_bits_set() {
    let state: u128 = 0xFFFFFFFF00000000FFFFFFFF00000000;
    let result = output_xsl_rr(state);
    assert_ne!(result, 0);
}

