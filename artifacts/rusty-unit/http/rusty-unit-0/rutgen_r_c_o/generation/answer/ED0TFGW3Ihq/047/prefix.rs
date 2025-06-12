// Answer 0

#[test]
fn test_parse_exact_https() {
    let input = b"https";
    let _ = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_https_plus() {
    let input = b"https+";
    let _ = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_https_minus() {
    let input = b"https-";
    let _ = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_https_with_digit() {
    let input = b"https0";
    let _ = Scheme2::<()>::parse_exact(input);
}

#[test]
fn test_parse_exact_https_with_numbers_and_letters() {
    let input = b"https1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ~";
    let _ = Scheme2::<()>::parse_exact(input);
}

