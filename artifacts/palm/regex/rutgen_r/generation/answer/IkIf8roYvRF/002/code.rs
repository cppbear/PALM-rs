// Answer 0

#[test]
fn test_case_insensitive_false() {
    struct Flags {
        case_insensitive: Option<bool>,
    }

    struct TranslatorBuilder {
        flags: Flags,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder {
                flags: Flags { case_insensitive: None },
            }
        }

        pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.case_insensitive = if yes { Some(true) } else { None };
            self
        }
    }

    // Initialize the TranslatorBuilder
    let mut builder = TranslatorBuilder::new();

    // Call the case_insensitive method with `yes` as false
    let result = builder.case_insensitive(false);

    // Assert that the case_insensitive flag is None
    assert!(result.flags.case_insensitive.is_none());
    // Assert that the returned value is the same instance as the original
    assert_eq!(result, &builder);
}

