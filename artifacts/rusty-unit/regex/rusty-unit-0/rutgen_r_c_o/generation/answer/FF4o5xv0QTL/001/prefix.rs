// Answer 0

#[test]
fn test_nfa_default_state() {
    let exec_builder = ExecBuilder::new("sample_regex");
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_bytes_true() {
    let exec_builder = ExecBuilder::new("sample_regex").bytes(true);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_bytes_false() {
    let exec_builder = ExecBuilder::new("sample_regex").bytes(false);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_only_utf8_true() {
    let exec_builder = ExecBuilder::new("sample_regex").only_utf8(true);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_only_utf8_false() {
    let exec_builder = ExecBuilder::new("sample_regex").only_utf8(false);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_unicode_true() {
    let exec_builder = ExecBuilder::new("sample_regex").unicode(true);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_unicode_false() {
    let exec_builder = ExecBuilder::new("sample_regex").unicode(false);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_all_options() {
    let exec_builder = ExecBuilder::new("sample_regex")
        .bytes(true)
        .only_utf8(false)
        .unicode(true);
    let result = exec_builder.nfa();
}

#[test]
fn test_nfa_with_all_options_reversed() {
    let exec_builder = ExecBuilder::new("sample_regex")
        .bytes(false)
        .only_utf8(true)
        .unicode(false);
    let result = exec_builder.nfa();
}

