// Answer 0

#[test]
fn test_multi_line_true() {
    struct TestTranslatorBuilder {
        flags: Flags,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TestTranslatorBuilder { flags: Flags::default() }
        }

        pub fn multi_line(&mut self, yes: bool) -> &mut Self {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TestTranslatorBuilder::new();
    let returned_builder = builder.multi_line(true);
    assert_eq!(returned_builder, &mut builder);
    assert_eq!(builder.flags.multi_line, Some(true));
}

#[test]
fn test_multi_line_false() {
    struct TestTranslatorBuilder {
        flags: Flags,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            TestTranslatorBuilder { flags: Flags::default() }
        }

        pub fn multi_line(&mut self, yes: bool) -> &mut Self {
            self.flags.multi_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TestTranslatorBuilder::new();
    let returned_builder = builder.multi_line(false);
    assert_eq!(returned_builder, &mut builder);
    assert_eq!(builder.flags.multi_line, None);
}

