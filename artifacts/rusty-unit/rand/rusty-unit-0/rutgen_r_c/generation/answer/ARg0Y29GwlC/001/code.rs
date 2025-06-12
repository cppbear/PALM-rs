// Answer 0

#[test]
fn test_read_u64_into_success() {
    let src: Vec<u8> = (0..16u64).flat_map(|n| n.to_le_bytes().to_vec()).collect(); // src has buffer for 2 u64
    let mut dst = vec![0u64; 2]; // 2 u64s will be filled

    read_u64_into(&src, &mut dst);

    assert_eq!(dst, vec![0, 1]); // Check the expected values
}

#[test]
#[should_panic]
fn test_read_u64_into_panic_insufficient_src_length() {
    let src: Vec<u8> = (0..10u64).flat_map(|n| n.to_le_bytes().to_vec()).collect(); // Not enough bytes for 2 u64s
    let mut dst = vec![0u64; 2]; // 2 u64s

    read_u64_into(&src, &mut dst); // This should panic
}

#[test]
#[should_panic]
fn test_read_u64_into_panic_on_invalid_chunk_conversion() {
    let src: Vec<u8> = vec![0u8; 15]; // Not enough full chunks to convert to u64
    let mut dst = vec![0u64; 2]; // 2 u64s will be filled

    read_u64_into(&src, &mut dst); // This should panic
}

