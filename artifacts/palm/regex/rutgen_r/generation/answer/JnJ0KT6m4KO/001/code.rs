// Answer 0

#[test]
fn test_freq_rank_valid_lower_bound() {
    let result = freq_rank(0);
    assert_eq!(result, BYTE_FREQUENCIES[0] as usize);
}

#[test]
fn test_freq_rank_valid_upper_bound() {
    let result = freq_rank(255);
    assert_eq!(result, BYTE_FREQUENCIES[255] as usize);
}

#[test]
#[should_panic]
fn test_freq_rank_invalid_below_lower_bound() {
    let _result = freq_rank(256);
}

#[test]
#[should_panic]
fn test_freq_rank_invalid_above_upper_bound() {
    let _result = freq_rank(255);
}

