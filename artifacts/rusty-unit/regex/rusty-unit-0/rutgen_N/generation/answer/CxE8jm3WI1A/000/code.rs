// Answer 0

#[derive(Default)]
struct TranslatorBuilder {
    flags: TranslatorFlags,
}

#[derive(Default)]
struct TranslatorFlags {
    dot_matches_new_line: Option<bool>,
}

impl TranslatorBuilder {
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
        self.flags.dot_matches_new_line = if yes { Some(true) } else { None };
        self
    }
}

#[test]
fn test_dot_matches_new_line_enabled() {
    let mut builder = TranslatorBuilder::default();
    builder.dot_matches_new_line(true);
    assert_eq!(builder.flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_dot_matches_new_line_disabled() {
    let mut builder = TranslatorBuilder::default();
    builder.dot_matches_new_line(false);
    assert_eq!(builder.flags.dot_matches_new_line, None);
}

