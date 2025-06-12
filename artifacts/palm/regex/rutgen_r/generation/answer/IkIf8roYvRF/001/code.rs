// Answer 0

#[test]
fn test_case_insensitive_enable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        case_insensitive: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.case_insensitive = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder {
        flags: Flags {
            case_insensitive: None,
        },
    };

    let result = builder.case_insensitive(true);
    assert_eq!(result, &mut builder);
    assert_eq!(builder.flags.case_insensitive, Some(true));
}

#[test]
fn test_case_insensitive_disable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        case_insensitive: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.case_insensitive = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder {
        flags: Flags {
            case_insensitive: Some(true),
        },
    };

    let result = builder.case_insensitive(false);
    assert_eq!(result, &mut builder);
    assert_eq!(builder.flags.case_insensitive, None);
}

