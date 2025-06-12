// Answer 0

#[test]
fn test_is_valid_cap_letter_valid_numbers() {
    let valid_numbers = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    for &b in valid_numbers.iter() {
        assert_eq!(is_valid_cap_letter(&b), true);
    }
}

#[test]
fn test_is_valid_cap_letter_valid_lowercase() {
    let valid_lowercase = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', 
                           b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', 
                           b't', b'u', b'v', b'w', b'x', b'y', b'z'];
    for &b in valid_lowercase.iter() {
        assert_eq!(is_valid_cap_letter(&b), true);
    }
}

#[test]
fn test_is_valid_cap_letter_valid_uppercase() {
    let valid_uppercase = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', 
                           b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', 
                           b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z'];
    for &b in valid_uppercase.iter() {
        assert_eq!(is_valid_cap_letter(&b), true);
    }
}

#[test]
fn test_is_valid_cap_letter_valid_underscore() {
    let underscore = b'_';
    assert_eq!(is_valid_cap_letter(&underscore), true);
}

#[test]
fn test_is_valid_cap_letter_invalid() {
    let invalid_bytes = [b'!', b'$', b'%', b'&', b'(', b')', b'*', b'+', b',', b'-', 
                         b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', 
                         b'[', b'\\', b']', b'^', b'{', b'|', b'}', b'~'];
    for &b in invalid_bytes.iter() {
        assert_eq!(is_valid_cap_letter(&b), false);
    }
}

