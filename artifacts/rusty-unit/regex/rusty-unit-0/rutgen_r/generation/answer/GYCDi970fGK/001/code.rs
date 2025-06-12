// Answer 0

#[test]
fn test_swap_greed_enabled() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        swap_greed: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder {
                flags: Flags { swap_greed: None },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.swap_greed(true);
    assert_eq!(result.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_disabled() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        swap_greed: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder {
                flags: Flags { swap_greed: None },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.swap_greed(false);
    assert_eq!(result.flags.swap_greed, None);
}

