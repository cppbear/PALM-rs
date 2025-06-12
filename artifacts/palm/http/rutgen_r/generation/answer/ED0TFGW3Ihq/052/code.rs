// Answer 0

#[test]
fn test_parse_exact_valid_other_scheme_max_length() {
    const MAX_SCHEME_LEN: usize = 10; // Assuming a predefined maximum length
    const SCHEME_CHARS: [u8; 256] = [1; 256]; // All allowable characters set to 1
    SCHEME_CHARS[b':' as usize] = 0; // Invalid character
    
    struct Scheme2<T>(T);
    
    #[derive(Debug)]
    struct Protocol;
    
    impl Protocol {
        fn Http() -> &'static str { "http" }
        fn Https() -> &'static str { "https" }
    }

    #[derive(Debug)]
    enum ErrorKind {
        SchemeTooLong,
        InvalidScheme,
    }

    #[derive(Debug)]
    struct InvalidUri(ErrorKind);

    fn parse_exact(s: &[u8]) -> Result<Scheme2<()>, InvalidUri> {
        match s {
            b"http" => Ok(Protocol::Http().into()),
            b"https" => Ok(Protocol::Https().into()),
            _ => {
                if s.len() > MAX_SCHEME_LEN {
                    return Err(InvalidUri(ErrorKind::SchemeTooLong));
                }
                for &b in s {
                    match SCHEME_CHARS[b as usize] {
                        b':' => {
                            return Err(InvalidUri(ErrorKind::InvalidScheme));
                        }
                        0 => {
                            return Err(InvalidUri(ErrorKind::InvalidScheme));
                        }
                        _ => {}
                    }
                }
                Ok(Scheme2::Other(()))
            }
        }
    }

    let test_input = b"valid_scheme"; // Valid input that does not match "http" or "https"
    let result = parse_exact(test_input);
    assert!(result.is_ok());
}

