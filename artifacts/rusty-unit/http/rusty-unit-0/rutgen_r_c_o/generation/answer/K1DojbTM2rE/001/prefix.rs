// Answer 0

#[test]
fn test_uninit_u8_array_zero() {
    let result: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = uninit_u8_array();
}

#[test]
fn test_uninit_u8_array_non_zero() {
    let result: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = uninit_u8_array();
} 

#[test]
#[should_panic]
fn test_uninit_u8_array_overflow() {
    let overflow_test: [MaybeUninit<u8>; SCRATCH_BUF_OVERFLOW] = uninit_u8_array();
}

