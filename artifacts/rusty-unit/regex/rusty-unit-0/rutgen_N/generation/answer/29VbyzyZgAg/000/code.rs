// Answer 0

#[derive(Default)]
struct Compiled {
    is_dfa: bool,
}

struct Regex {
    compiled: Compiled,
}

impl Regex {
    pub fn dfa(mut self, yes: bool) -> Self {
        self.comcompiled.is_dfa = yes;
        self
    }
}

#[test]
fn test_dfa_sets_is_dfa_to_true() {
    let regex = Regex::default();
    let updated_regex = regex.dfa(true);
    assert!(updated_regex.compiled.is_dfa);
}

#[test]
fn test_dfa_sets_is_dfa_to_false() {
    let regex = Regex::default();
    let updated_regex = regex.dfa(false);
    assert!(!updated_regex.compiled.is_dfa);
}

#[test]
fn test_dfa_chainability() {
    let regex = Regex::default();
    let updated_regex = regex.dfa(true).dfa(false);
    assert!(!updated_regex.compiled.is_dfa);
}

