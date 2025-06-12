// Answer 0

#[test]
fn test_multi_line_setting_false() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Flags {
        multi_line: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> TranslatorBuilder {
            TranslatorBuilder {
                flags: Flags::default(),
            }
        }

        pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.multi_line(false);
    assert_eq!(result.flags.multi_line, None);
}

#[test]
fn test_multi_line_setting_true() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Flags {
        multi_line: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> TranslatorBuilder {
            TranslatorBuilder {
                flags: Flags::default(),
            }
        }

        pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.multi_line(true);
    assert_eq!(result.flags.multi_line, Some(true));
}

