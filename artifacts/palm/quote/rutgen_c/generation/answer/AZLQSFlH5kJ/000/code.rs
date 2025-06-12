// Answer 0

#[test]
fn test_fmt_for_ident_fragment_with_string() {
    struct StringFragment(String);
    impl IdentFragment for StringFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    let fragment = StringFragment("test".to_string());
    let mut output = String::new();
    let result = fragment.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "test");
}

#[test]
fn test_fmt_for_ident_fragment_with_u32() {
    struct U32Fragment(u32);
    impl IdentFragment for U32Fragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let fragment = U32Fragment(42);
    let mut output = String::new();
    let result = fragment.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "42");
}

#[test]
fn test_fmt_for_ident_fragment_with_char() {
    struct CharFragment(char);
    impl IdentFragment for CharFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let fragment = CharFragment('c');
    let mut output = String::new();
    let result = fragment.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "c");
}

#[test]
fn test_fmt_for_ident_fragment_with_bool() {
    struct BoolFragment(bool);
    impl IdentFragment for BoolFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let fragment = BoolFragment(true);
    let mut output = String::new();
    let result = fragment.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "true");
}

