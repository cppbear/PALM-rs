// Answer 0

#[test]
fn test_fmt_state_flags() {
    struct StateFlags {
        is_match_value: bool,
        is_word_value: bool,
        has_empty_value: bool,
    }

    impl StateFlags {
        fn is_match(&self) -> bool {
            self.is_match_value
        }

        fn is_word(&self) -> bool {
            self.is_word_value
        }

        fn has_empty(&self) -> bool {
            self.has_empty_value
        }
    }

    use std::fmt;

    impl fmt::Debug for StateFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("StateFlags")
             .field("is_match", &self.is_match())
             .field("is_word", &self.is_word())
             .field("has_empty", &self.has_empty())
             .finish()
        }
    }

    let state_flags = StateFlags {
        is_match_value: true,
        is_word_value: false,
        has_empty_value: true,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);

    assert!(result.is_ok());
    assert!(output.contains("is_match: true"));
    assert!(output.contains("is_word: false"));
    assert!(output.contains("has_empty: true"));
}

#[test]
fn test_fmt_state_flags_empty() {
    struct StateFlags {
        is_match_value: bool,
        is_word_value: bool,
        has_empty_value: bool,
    }

    impl StateFlags {
        fn is_match(&self) -> bool {
            self.is_match_value
        }

        fn is_word(&self) -> bool {
            self.is_word_value
        }

        fn has_empty(&self) -> bool {
            self.has_empty_value
        }
    }

    use std::fmt;

    impl fmt::Debug for StateFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("StateFlags")
             .field("is_match", &self.is_match())
             .field("is_word", &self.is_word())
             .field("has_empty", &self.has_empty())
             .finish()
        }
    }

    let state_flags = StateFlags {
        is_match_value: false,
        is_word_value: false,
        has_empty_value: false,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", state_flags);

    assert!(result.is_ok());
    assert!(output.contains("is_match: false"));
    assert!(output.contains("is_word: false"));
    assert!(output.contains("has_empty: false"));
}

