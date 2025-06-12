// Answer 0

#[test]
fn test_decode_helper_valid_input() {
    let input = b"SGVsbG8sIFdvcmxkIQ==";
    let estimate = GeneralPurposeEstimate {
        rem: 2,
        conservative_decoded_len: 20,
    };
    let mut output = vec![0u8; 20];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // A = 0
        table[b'B' as usize] = 1; // B = 1
        table[b'C' as usize] = 2; // C = 2
        table[b'D' as usize] = 3; // D = 3
        table[b'E' as usize] = 4; // E = 4
        table[b'F' as usize] = 5; // F = 5
        table[b'G' as usize] = 6; // G = 6
        // ... fill in rest of base64 table
        table[b'S' as usize] = 18; // S = 18
        table[b'T' as usize] = 19; // T = 19
        table[b'U' as usize] = 20; // U = 20
        table[b'V' as usize] = 21; // V = 21
        table[b'W' as usize] = 22; // W = 22
        table[b'X' as usize] = 23; // X = 23
        table[b'Y' as usize] = 24; // Y = 24
        table[b'Z' as usize] = 25; // Z = 25
        table[b'a' as usize] = 26; // a = 26
        table[b'b' as usize] = 27; // b = 27
        table[b'c' as usize] = 28; // c = 28
        table[b'd' as usize] = 29; // d = 29
        table[b'e' as usize] = 30; // e = 30
        table[b'f' as usize] = 31; // f = 31
        table[b'g' as usize] = 32; // g = 32
        table[b'h' as usize] = 33; // h = 33
        table[b'i' as usize] = 34; // i = 34
        table[b'j' as usize] = 35; // j = 35
        table[b'k' as usize] = 36; // k = 36
        table[b'l' as usize] = 37; // l = 37
        table[b'm' as usize] = 38; // m = 38
        table[b'n' as usize] = 39; // n = 39
        table[b'o' as usize] = 40; // o = 40
        table[b'p' as usize] = 41; // p = 41
        table[b'q' as usize] = 42; // q = 42
        table[b'r' as usize] = 43; // r = 43
        table[b's' as usize] = 44; // s = 44
        table[b't' as usize] = 45; // t = 45
        table[b'u' as usize] = 46; // u = 46
        table[b'v' as usize] = 47; // v = 47
        table[b'w' as usize] = 48; // w = 48
        table[b'x' as usize] = 49; // x = 49
        table[b'y' as usize] = 50; // y = 50
        table[b'z' as usize] = 51; // z = 51
        table[b'0' as usize] = 52; // 0 = 52
        table[b'1' as usize] = 53; // 1 = 53
        table[b'2' as usize] = 54; // 2 = 54
        table[b'3' as usize] = 55; // 3 = 55
        table[b'4' as usize] = 56; // 4 = 56
        table[b'5' as usize] = 57; // 5 = 57
        table[b'6' as usize] = 58; // 6 = 58
        table[b'7' as usize] = 59; // 7 = 59
        table[b'8' as usize] = 60; // 8 = 60
        table[b'9' as usize] = 61; // 9 = 61
        table[b'+' as usize] = 62; // + = 62
        table[b'/' as usize] = 63; // / = 63
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 13);
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_byte() {
    let input = b"SGVsbG8sIFdvcmxkIQ==*"; // Invalid byte at the end
    let estimate = GeneralPurposeEstimate {
        rem: 2,
        conservative_decoded_len: 20,
    };
    let mut output = vec![0u8; 20];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // A = 0
        table[b'B' as usize] = 1; // B = 1
        // ... fill in valid base64 table as required
        table
    };
    let _ = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
}

#[test]
#[should_panic]
fn test_decode_helper_too_short_output() {
    let input = b"SGVsbG8sIFdvcmxkIQ=="; // Valid input
    let estimate = GeneralPurposeEstimate {
        rem: 2,
        conservative_decoded_len: 20,
    };
    let mut output = vec![0u8; 5]; // Output slice too small
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // A = 0
        table[b'B' as usize] = 1; // B = 1
        // ... fill in valid base64 table as required
        table
    };
    let _ = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
}

