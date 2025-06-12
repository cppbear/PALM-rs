// Answer 0

#[test]
fn test_valid_scheme_standard() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("http");
    assert!(scheme.is_ok());
}

#[test]
fn test_valid_scheme_other() {
    struct CustomByteStr(Box<ByteStr>);
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("ftp");
    assert!(scheme.is_ok());
    
    let scheme_other: Result<Scheme, InvalidUri> = Scheme::from_str("custom-scheme");
    assert!(scheme_other.is_ok());
}

#[test]
fn test_invalid_scheme_empty() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("");
    assert!(scheme.is_err());
}

#[test]
fn test_invalid_scheme_too_long() {
    let long_scheme = "a".repeat(65); // 65 characters long
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str(&long_scheme);
    assert!(scheme.is_err());
}

#[test]
fn test_invalid_scheme_invalid_characters() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("http://invalid");
    assert!(scheme.is_err());
} 

#[test]
fn test_invalid_scheme_non_ascii() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("http 日本語");
    assert!(scheme.is_err());
} 

#[test]
fn test_valid_scheme_with_special_chars() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("custom+scheme");
    assert!(scheme.is_ok());
}

#[test]
fn test_scheme_with_excessively_long_name() {
    let too_long_scheme = "x".repeat(100);
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str(&too_long_scheme);
    assert!(scheme.is_err());
}

#[test]
fn test_scheme_with_non_alphanumeric() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("http!@#");
    assert!(scheme.is_err());
} 

#[test]
fn test_scheme_with_whitespace() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str(" http ");
    assert!(scheme.is_err());
} 

#[test]
fn test_scheme_with_mixed_case() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::from_str("Http");
    assert!(scheme.is_ok());
}

