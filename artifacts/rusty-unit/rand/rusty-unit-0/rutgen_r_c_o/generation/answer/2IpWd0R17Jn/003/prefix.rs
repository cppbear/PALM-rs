// Answer 0

#[test]
#[should_panic]
fn test_read_u32_into_panic_short_source_length() {
    let src: [u8; 3] = [1, 2, 3]; // 3 bytes
    let mut dst: [u32; 1] = [0]; // 1 u32 requires 4 bytes
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_panic_zero_source_length() {
    let src: [u8; 0] = []; // 0 bytes
    let mut dst: [u32; 1] = [0]; // 1 u32 requires 4 bytes
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_panic_short_destination_length() {
    let src: [u8; 2] = [1, 2]; // 2 bytes
    let mut dst: [u32; 1] = [0]; // 1 u32 requires 4 bytes
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_panic_exceeding_source_length() {
    let src: [u8; 1] = [1]; // 1 byte
    let mut dst: [u32; 2] = [0, 0]; // 2 u32 requires 8 bytes
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_panic_exactly_short() {
    let src: [u8; 4] = [1, 2, 3, 4]; // 4 bytes
    let mut dst: [u32; 2] = [0, 0]; // 2 u32 requires 8 bytes
    read_u32_into(&src, &mut dst);
}

