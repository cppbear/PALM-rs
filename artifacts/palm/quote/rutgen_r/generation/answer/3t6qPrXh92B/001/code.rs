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
        
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                std::fmt::Display::fmt(id, f)
            } else {
                std::fmt::Display::fmt(&id[..], f)
            }
        }
    }

    let id_fragment = IdentFragment { value: "r#identifier".to_string() };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", id_fragment);
    assert!(result.is_ok());
    assert_eq!(buffer, "identifier");
}

#[test]
#[should_panic]
fn test_fmt_incorrect_slice() {
    struct IdentFragment {
        value: String,
    }

    impl IdentFragment {
        fn to_string(&self) -> String {
            self.value.clone()
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let id = self.to_string();
            if let Some(id) = id.strip_prefix("r#") {
                std::fmt::Display::fmt(id, f)
            } else {
                // This panic is intended for the test
                std::fmt::Display::fmt(&id[..0], f)
            }
        }
    }

    let id_fragment = IdentFragment { value: "identifier".to_string() };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", id_fragment);
}

