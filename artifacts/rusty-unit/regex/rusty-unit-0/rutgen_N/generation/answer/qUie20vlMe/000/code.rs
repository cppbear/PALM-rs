// Answer 0

#[derive(Default)]
struct Options {
    unicode: bool,
}

struct Regex {
    options: Options,
}

impl Regex {
    pub fn unicode(mut self, yes: bool) -> Self {
        self.options.unicode = yes;
        self
    }
}

#[test]
fn test_unicode_set_to_true() {
    let regex = Regex::default();
    let updated_regex = regex.unicode(true);
    assert!(updated_regex.options.unicode);
}

#[test]
fn test_unicode_set_to_false() {
    let regex = Regex::default();
    let updated_regex = regex.unicode(false);
    assert!(!updated_regex.options.unicode);
}

