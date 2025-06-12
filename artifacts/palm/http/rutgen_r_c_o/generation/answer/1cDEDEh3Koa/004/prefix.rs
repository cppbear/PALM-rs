// Answer 0

#[test]
fn test_from_bytes_invalid_condition_a_zero() {
    let input: &[u8] = &[b'0', b'9', b'5'];
    let _ = StatusCode::from_bytes(input);
}

#[test]
fn test_from_bytes_invalid_condition_b_gt_9() {
    let input: &[u8] = &[b'9', b'1', b'0'];
    let _ = StatusCode::from_bytes(input);
}

