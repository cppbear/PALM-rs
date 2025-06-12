// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct CapturesDebug<'a>(&'a str);

    impl fmt::Debug for CapturesDebug<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    let captures = CapturesDebug("test_captures");
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        captures.fmt(formatter).unwrap();
    }

    assert_eq!(output, "Captures(test_captures)");
}

#[test]
fn test_empty_fmt() {
    use std::fmt;

    struct CapturesDebug<'a>(&'a str);

    impl fmt::Debug for CapturesDebug<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    let captures = CapturesDebug("");
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        captures.fmt(formatter).unwrap();
    }

    assert_eq!(output, "Captures()");
}

