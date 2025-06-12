// Answer 0

#[test]
fn test_fmt() {
    struct IdentFragment {
        value: String,
    }
    
    impl IdentFragment {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }
    
    let ident_fragment = IdentFragment {
        value: String::from("test_identifier"),
    };

    let mut output = String::new();
    let result = ident_fragment.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "test_identifier");
}

#[test]
fn test_fmt_empty() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }
    
    let ident_fragment = IdentFragment {
        value: String::from(""),
    };

    let mut output = String::new();
    let result = ident_fragment.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "");
}

