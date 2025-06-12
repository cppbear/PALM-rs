// Answer 0

#[test]
fn test_multi_line_enable() {
    // Define a struct that contains the flags
    struct Flags {
        multi_line: Option<bool>,
    }

    struct TranslatorBuilder {
        flags: Flags,
    }

    // Implement the multi_line method
    impl TranslatorBuilder {
        pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    // Create a TranslatorBuilder instance
    let mut builder = TranslatorBuilder {
        flags: Flags { multi_line: None },
    };

    // Set the multi-line flag to true
    let result = builder.multi_line(true);

    // Assert that the multi-line flag is enabled
    assert_eq!(result.flags.multi_line, Some(true));
}

#[test]
fn test_multi_line_disable() {
    struct Flags {
        multi_line: Option<bool>,
    }

    struct TranslatorBuilder {
        flags: Flags,
    }

    impl TranslatorBuilder {
        pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder {
        flags: Flags { multi_line: None },
    };

    let result = builder.multi_line(false);

    assert_eq!(result.flags.multi_line, None);
}

