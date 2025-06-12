// Answer 0

#[test]
fn test_fmt_http_3() {
    use std::fmt;

    struct Http(u8);
    
    impl Http {
        const H3: u8 = 5; // Assuming H3 is represented by some unique value, e.g., 5
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                Http::H3 => f.write_str("HTTP/3.0"),
                _ => unreachable!(),
            }
        }
    }

    let http_version = Http(Http::H3);
    let mut output = String::new();
    let result = write!(&mut output, "{}", http_version);
    
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/3.0");
}

#[test]
#[should_panic]
fn test_fmt_unreachable_case() {
    use std::fmt;

    struct Http(u8);
    
    impl Http {
        const H3: u8 = 5;
    }

    impl fmt::Display for Http {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                Http::H3 => f.write_str("HTTP/3.0"),
                _ => unreachable!(),
            }
        }
    }

    let http_version = Http(0); // This will trigger the unreachable!() panic
    let _ = format!("{}", http_version);
}

