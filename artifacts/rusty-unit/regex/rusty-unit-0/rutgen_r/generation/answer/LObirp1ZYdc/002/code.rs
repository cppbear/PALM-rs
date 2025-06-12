// Answer 0

#[test]
fn test_multi_line_disable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        multi_line: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn multi_line(&mut self, yes: bool) -> &mut Self {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder {
        flags: Flags { multi_line: None },
    };

    // Perform the test with `yes` set to false
    let result = builder.multi_line(false);

    // Assert that the return value is the same as the builder
    assert_eq!(result as *const _, &builder as *const _);

    // Assert that the multi_line flag is set to None
    assert!(builder.flags.multi_line.is_none());
}

