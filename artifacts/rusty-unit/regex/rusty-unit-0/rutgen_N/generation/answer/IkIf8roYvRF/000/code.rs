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
        pub fn new() -> Self {
            Self {
                flags: Flags {
                    case_insensitive: None,
                },
            }
        }
        
        pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.case_insensitive = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(true);
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
        pub fn new() -> Self {
            Self {
                flags: Flags {
                    case_insensitive: None,
                },
            }
        }
        
        pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.case_insensitive = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(false);
    assert_eq!(builder.flags.case_insensitive, None);
}

