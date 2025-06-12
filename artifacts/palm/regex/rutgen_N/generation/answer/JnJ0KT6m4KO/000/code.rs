// Answer 0

#[test]
fn test_freq_rank_valid_byte() {
    let result = freq_rank(0x61); // ASCII for 'a'
    assert_eq!(result, BYTE_FREQUENCIES[0x61 as usize] as usize);
}

#[test]
fn test_freq_rank_boundary_low() {
    let result = freq_rank(0x00); // Lowest byte value
    assert_eq!(result, BYTE_FREQUENCIES[0x00 as usize] as usize);
}

#[test]
fn test_freq_rank_boundary_high() {
    let result = freq_rank(0xff); // Highest byte value
    assert_eq!(result, BYTE_FREQUENCIES[0xff as usize] as usize);
}

#[test]
#[should_panic]
fn test_freq_rank_out_of_bounds() {
    let _result = freq_rank(0x100); // Out of bounds, should panic
}

