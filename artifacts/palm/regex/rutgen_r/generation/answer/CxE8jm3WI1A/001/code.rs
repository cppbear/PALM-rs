// Answer 0

#[test]
fn test_dot_matches_new_line_true() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        dot_matches_new_line: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            Self {
                flags: Flags { dot_matches_new_line: None },
            }
        }

        pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.dot_matches_new_line(true);
    assert_eq!(result.flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_dot_matches_new_line_false() {
    struct TranslatorBuilder {
        flags: Flags,
    }

    struct Flags {
        dot_matches_new_line: Option<bool>,
    }

    impl TranslatorBuilder {
        pub fn new() -> Self {
            Self {
                flags: Flags { dot_matches_new_line: None },
            }
        }

        pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {
            self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
            self
        }
    }

    let mut builder = TranslatorBuilder::new();
    let result = builder.dot_matches_new_line(false);
    assert_eq!(result.flags.dot_matches_new_line, None);
}

