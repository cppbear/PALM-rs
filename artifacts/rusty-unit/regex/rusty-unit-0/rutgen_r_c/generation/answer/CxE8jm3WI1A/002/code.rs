// Answer 0

#[test]
fn test_dot_matches_new_line_false() {
    struct DummySpan;
    struct DummyFlags {
        dot_matches_new_line: Option<bool>,
    }

    #[derive(Clone, Debug)]
    struct DummyTranslatorBuilder {
        flags: DummyFlags,
    }

    impl DummyTranslatorBuilder {
        fn new() -> Self {
            Self {
                flags: DummyFlags { dot_matches_new_line: None },
            }
        }
        
        fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
            self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = DummyTranslatorBuilder::new();
    let result = builder.dot_matches_new_line(false);
    assert_eq!(result.flags.dot_matches_new_line, None);
}

#[test]
fn test_dot_matches_new_line_true() {
    struct DummySpan;
    struct DummyFlags {
        dot_matches_new_line: Option<bool>,
    }

    #[derive(Clone, Debug)]
    struct DummyTranslatorBuilder {
        flags: DummyFlags,
    }

    impl DummyTranslatorBuilder {
        fn new() -> Self {
            Self {
                flags: DummyFlags { dot_matches_new_line: None },
            }
        }
        
        fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
            self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = DummyTranslatorBuilder::new();
    let result = builder.dot_matches_new_line(true);
    assert_eq!(result.flags.dot_matches_new_line, Some(true));
}

