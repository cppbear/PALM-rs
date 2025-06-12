// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[derive(Debug)]
    struct Status(u16);

    impl Status {
        fn canonical_reason(&self) -> Option<&'static str> {
            match self.0 {
                200 => Some("OK"),
                404 => Some("Not Found"),
                _ => None,
            }
        }
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} {}",
                u16::from(self.0),
                self.canonical_reason().unwrap_or("<unknown status code>")
            )
        }
    }

    #[test]
    fn test_fmt_200_status() {
        let status = Status(200);
        let mut output = String::new();
        let fmt_result = write!(&mut output, "{}", status);
        assert!(fmt_result.is_ok());
        assert_eq!(output, "200 OK");
    }

    #[test]
    fn test_fmt_404_status() {
        let status = Status(404);
        let mut output = String::new();
        let fmt_result = write!(&mut output, "{}", status);
        assert!(fmt_result.is_ok());
        assert_eq!(output, "404 Not Found");
    }

    #[test]
    fn test_fmt_unknown_status() {
        let status = Status(500);
        let mut output = String::new();
        let fmt_result = write!(&mut output, "{}", status);
        assert!(fmt_result.is_ok());
        assert_eq!(output, "500 <unknown status code>");
    }
}

