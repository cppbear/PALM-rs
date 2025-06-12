// Answer 0

fn unicode_test() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        unicode: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TranslatorBuilder { flags: Flags { unicode: None } }
        }

        pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.unicode = if yes { None } else { Some(false) };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.unicode(false);

    assert_eq!(result.flags.unicode, Some(false));
}

