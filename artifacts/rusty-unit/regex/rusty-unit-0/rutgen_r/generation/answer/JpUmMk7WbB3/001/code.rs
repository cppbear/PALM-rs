// Answer 0

#[test]
fn test_fmt_with_valid_string() {
    struct RegexStr<'a> {
        original: &'a str,
    }

    impl<'a> RegexStr<'a> {
        fn as_str(&self) -> &'a str {
            self.original
        }
    }

    let regex = RegexStr { original: r"(a|b)*" };
    let mut output = String::new();
    let result = regex.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, r"(a|b)*");
}

#[test]
fn test_fmt_with_empty_string() {
    struct RegexStr<'a> {
        original: &'a str,
    }

    impl<'a> RegexStr<'a> {
        fn as_str(&self) -> &'a str {
            self.original
        }
    }

    let regex = RegexStr { original: "" };
    let mut output = String::new();
    let result = regex.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "");
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_state() {
    struct RegexStr<'a> {
        original: &'a str,
        invalid_state: bool,
    }

    impl<'a> RegexStr<'a> {
        fn as_str(&self) -> &'a str {
            if self.invalid_state {
                panic!("Invalid state reached");
            }
            self.original
        }
    }

    let regex = RegexStr { original: r"abc", invalid_state: true };
    let mut output = String::new();
    let _ = regex.fmt(&mut output);
}

