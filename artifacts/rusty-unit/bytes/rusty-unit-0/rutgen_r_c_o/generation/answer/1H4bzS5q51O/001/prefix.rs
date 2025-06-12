// Answer 0

#[test]
fn test_copy_to_slice_panics_when_remaining_is_zero_and_dst_length_is_one() {
    let mut buf: &mut [u8] = &mut [];
    let mut dst = [0; 1];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_panics_when_remaining_is_less_than_dst_length() {
    let mut buf: &mut [u8] = &mut [1];
    let mut dst = [0; 2];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_copies_one_byte() {
    let mut buf: &mut [u8] = &mut [42];
    let mut dst = [0; 1];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_copies_two_bytes() {
    let mut buf: &mut [u8] = &mut [1, 2];
    let mut dst = [0; 2];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_copies_three_bytes() {
    let mut buf: &mut [u8] = &mut [1, 2, 3];
    let mut dst = [0; 3];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_copies_buffer_of_exact_length() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4, 5];
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_fills_dst_with_remaining_bytes() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
    let mut dst = [0; 3];
    buf.copy_to_slice(&mut dst);
}

#[test]
fn test_copy_to_slice_multiple_copies() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
    let mut dst1 = [0; 3];
    let mut dst2 = [0; 3];
    buf.copy_to_slice(&mut dst1);
    buf.copy_to_slice(&mut dst2);
}

