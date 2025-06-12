// Answer 0

#[test]
fn test_fmt_http_scheme() {
    let http_scheme = Scheme::HTTP;
    let mut output = String::new();
    let result = write!(&mut output, "{}", http_scheme);
    assert!(result.is_ok());
    assert_eq!(output, "http");
}

#[test]
fn test_fmt_https_scheme() {
    let https_scheme = Scheme::HTTPS;
    let mut output = String::new();
    let result = write!(&mut output, "{}", https_scheme);
    assert!(result.is_ok());
    assert_eq!(output, "https");
}

#[test]
fn test_fmt_empty_scheme() {
    let empty_scheme = Scheme::empty();
    let mut output = String::new();
    let result = write!(&mut output, "{}", empty_scheme);
    assert!(result.is_err());
}

#[test]
fn test_fmt_other_scheme() {
    struct CustomByteStr {
        data: Vec<u8>,
    }
    
    impl AsRef<[u8]> for CustomByteStr {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    let custom_data = CustomByteStr { data: b"custom-scheme".to_vec() };
    let other_scheme = Scheme {
        inner: Scheme2::Other(Box::new(custom_data)),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", other_scheme);
    assert!(result.is_ok());
    assert_eq!(output, "custom-scheme");
}

