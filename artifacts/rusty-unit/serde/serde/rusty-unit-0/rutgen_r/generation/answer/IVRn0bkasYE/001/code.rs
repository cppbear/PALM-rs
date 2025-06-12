// Answer 0

#[test]
fn test_format_u8_boundary_case() {
    let n: u8 = 100; // constraint: n >= 100 is true, with bound n == 100
    let mut out = [0u8; 3]; // allocate enough space for the output
    let result = format_u8(n, &mut out);
    assert_eq!(result, 3);
    assert_eq!(out, [b'1', b'0', b'0']); // expected output for n == 100
}

