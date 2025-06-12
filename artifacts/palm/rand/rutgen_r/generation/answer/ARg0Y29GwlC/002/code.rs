// Answer 0

#[test]
fn test_read_u64_into_success() {
    let src: Vec<u8> = (0..64).map(|i| i as u8).collect(); // 64 bytes, enough for 8 u64 values
    let mut dst = [0u64; 8]; // Will store 8 u64 values

    read_u64_into(&src, &mut dst);
    
    assert_eq!(dst, [
        0x0000000001020304,
        0x0000000506070809,
        0x0000000a0b0c0d0e,
        0x0000000f10111213,
        0x0000001415161718,
        0x0000001a1b1c1d1e,
        0x0000001f20212223,
        0x0000002425262728
    ]);
}

#[test]
#[should_panic]
fn test_read_u64_into_insufficient_src_length() {
    let src: Vec<u8> = (0..40).map(|i| i as u8).collect(); // 40 bytes, not enough for 8 u64 values
    let mut dst = [0u64; 8]; // Will store 8 u64 values

    read_u64_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_empty_dst() {
    let src: Vec<u8> = (0..8).map(|i| i as u8).collect(); // 8 bytes, enough for 1 u64 value
    let mut dst: [u64; 0] = []; // Empty dst

    read_u64_into(&src, &mut dst);
}

