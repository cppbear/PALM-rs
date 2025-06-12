// Answer 0

#[test]
fn test_swap_greed_enable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        swap_greed: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            Self {
                flags: Flags { swap_greed: None },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(true);
    assert_eq!(builder.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_disable() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        swap_greed: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            Self {
                flags: Flags { swap_greed: None },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(false);
    assert_eq!(builder.flags.swap_greed, None);
}

