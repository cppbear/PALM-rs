// Answer 0

#[test]
fn test_swap_greed_true() {
    struct MockTranslatorBuilder {
        flags: Flags,
    }

    impl MockTranslatorBuilder {
        pub fn new() -> MockTranslatorBuilder {
            MockTranslatorBuilder {
                flags: Flags {
                    swap_greed: None,
                    ..Default::default()
                },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut MockTranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = MockTranslatorBuilder::new();
    let result = builder.swap_greed(true);
    assert_eq!(result.flags.swap_greed, Some(true));
}

#[test]
fn test_swap_greed_false() {
    struct MockTranslatorBuilder {
        flags: Flags,
    }

    impl MockTranslatorBuilder {
        pub fn new() -> MockTranslatorBuilder {
            MockTranslatorBuilder {
                flags: Flags {
                    swap_greed: None,
                    ..Default::default()
                },
            }
        }

        pub fn swap_greed(&mut self, yes: bool) -> &mut MockTranslatorBuilder {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = MockTranslatorBuilder::new();
    let result = builder.swap_greed(false);
    assert_eq!(result.flags.swap_greed, None);
}

