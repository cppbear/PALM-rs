// Answer 0

#[test]
fn test_byte() {
    struct Byte(u16);

    let input_0: u8 = 0;          // boundary value
    let expected_0 = Byte(input_0 as u16);
    assert_eq!(byte(input_0), expected_0);

    let input_1: u8 = 255;        // maximum value
    let expected_1 = Byte(input_1 as u16);
    assert_eq!(byte(input_1), expected_1);

    let input_2: u8 = 128;        // middle value
    let expected_2 = Byte(input_2 as u16);
    assert_eq!(byte(input_2), expected_2);
}

