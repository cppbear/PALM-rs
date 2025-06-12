// Answer 0

#[test]
fn test_swap_greed_false() {
    struct DummyTranslatorBuilder {
        flags: DummyFlags,
    }

    #[derive(Default)]
    struct DummyFlags {
        swap_greed: Option<bool>,
    }

    impl DummyTranslatorBuilder {
        fn new() -> Self {
            DummyTranslatorBuilder {
                flags: DummyFlags::default(),
            }
        }

        fn swap_greed(&mut self, yes: bool) -> &mut Self {
            self.flags.swap_greed = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = DummyTranslatorBuilder::new();
    let result = builder.swap_greed(false);

    assert_eq!(result.flags.swap_greed, None);
}

