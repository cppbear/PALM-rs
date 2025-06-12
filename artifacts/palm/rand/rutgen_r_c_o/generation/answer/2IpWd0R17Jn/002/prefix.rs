// Answer 0

#[test]
fn test_read_u32_into_zero_length_dst() {
    let src = [1u8, 0, 0, 0];
    let mut dst: Vec<u32> = Vec::new();
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_insufficient_src_length() {
    let src = [1u8];
    let mut dst = [0u32; 1];
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_single_element_dst() {
    let src = [1u8, 0, 0, 0];
    let mut dst = [0u32; 1];
    read_u32_into(&src, &mut dst);
}

#[test]
fn test_read_u32_into_multiple_elements_dst() {
    let src = [1u8, 0, 0, 0, 2u8, 0, 0, 0];
    let mut dst = [0u32; 2];
    read_u32_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u32_into_edge_case_panic() {
    let src = [1u8, 0, 0, 0, 2u8, 0, 0];
    let mut dst = [0u32; 2];
    read_u32_into(&src, &mut dst);
}

