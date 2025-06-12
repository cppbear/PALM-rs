// Answer 0

#[test]
fn test_freq_rank_valid_input_lower_bound() {
    let b: u8 = 0;
    let result = freq_rank(b);
    assert_eq!(result, BYTE_FREQUENCIES[b as usize] as usize);
}

#[test]
fn test_freq_rank_valid_input_upper_bound() {
    let b: u8 = 255;
    let result = freq_rank(b);
    assert_eq!(result, BYTE_FREQUENCIES[b as usize] as usize);
}

#[test]
#[should_panic]
fn test_freq_rank_out_of_bounds_negative() {
    let b: u8 = 256; // This will not actually compile since u8 cannot be 256.
}

#[test]
#[should_panic]
fn test_freq_rank_out_of_bounds_positive() {
    let b: u8 = 300; // This will not actually compile since u8 cannot be higher than 255.
}

