// Answer 0

#[test]
fn test_fmt_with_r_prefix() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn to_string(&self) -> String {
            self.value.clone()
        }
    }

    use std::fmt::{self, Formatter};

    impl std::fmt::Display for IdentFragment {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                fmt::Display::fmt(id, f)
            } else {
                fmt::Display::fmt(&id[..], f)
            }
        }
    }

    let fragment = IdentFragment { value: String::from("r#example") };
    let mut output = String::new();
    let result = write!(&mut output, "{}", fragment);
    assert!(result.is_ok());
    assert_eq!(output, "example");
}

#[test]
fn test_fmt_without_r_prefix() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn to_string(&self) -> String {
            self.value.clone()
        }
    }

    use std::fmt::{self, Formatter};

    impl std::fmt::Display for IdentFragment {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                fmt::Display::fmt(id, f)
            } else {
                fmt::Display::fmt(&id[..], f)
            }
        }
    }

    let fragment = IdentFragment { value: String::from("example") };
    let mut output = String::new();
    let result = write!(&mut output, "{}", fragment);
    assert!(result.is_ok());
    assert_eq!(output, "example");
}

