// Answer 0

#[test]
fn test_unicode_true() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        unicode: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder {
                flags: Flags { unicode: None },
            }
        }

        pub fn unicode(&mut self, yes: bool) -> &mut Self {
            self.flags.unicode = if yes { None } else { Some(false) };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.unicode(true);
    assert_eq!(result as *const _, &mut builder as *mut _);
}

#[test]
fn test_unicode_false() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        unicode: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder {
                flags: Flags { unicode: None },
            }
        }

        pub fn unicode(&mut self, yes: bool) -> &mut Self {
            self.flags.unicode = if yes { None } else { Some(false) };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.unicode(false);
    assert_eq!(result as *const _, &mut builder as *mut _);
    assert_eq!(builder.flags.unicode, Some(false));
}

