// Answer 0

#[test]
fn test_decode_four_hex_digits_none() {
    // The inputs for this test are chosen to ensure the calculated codepoint is negative.
    // We can use upper values that lead to an invalid character beyond the extended range for codepoint.
    
    let result = decode_four_hex_digits(0x10, 0x0F, 0x10, 0x0F); // HEX1[16], HEX0[15]
    assert_eq!(result, None);
} 

#[test]
fn test_decode_four_hex_digits_boundary_neg() {
    // Using values that result in a codepoint that is exactly negative, 
    // for example, utilizing high values for inputs to cross the signed boundary.
    
    let result = decode_four_hex_digits(0x0F, 0x0F, 0x0F, 0x0F); // HEX1[15], HEX0[15]
    assert_eq!(result, None);
} 

#[test]
fn test_decode_four_hex_digits_invalid_high() {
    // This test ensures to check combinations that will also lead to an invalid codepoint.
    
    let result = decode_four_hex_digits(0xF0, 0xF0, 0xF0, 0xF0); // HEX1[240], HEX0[240]
    assert_eq!(result, None);
} 

