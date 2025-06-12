// Answer 0

#[test]
fn test_nfa_setting() {
    let re = "example regex";
    let exec_builder = ExecBuilder::new(re);
    
    // Test that nfa sets the match type to Nfa(PikeVM)
    let updated_builder = exec_builder.nfa();
    assert_eq!(updated_builder.match_type, Some(MatchType::Nfa(MatchNfaType::PikeVM)));
}

#[test]
fn test_nfa_overrides_automatic() {
    let re = "example regex";
    let exec_builder = ExecBuilder::new(re).automatic();
    
    // After calling nfa, it should still be Nfa(PikeVM)
    let updated_builder = exec_builder.nfa();
    assert_eq!(updated_builder.match_type, Some(MatchType::Nfa(MatchNfaType::PikeVM)));
}

#[test]
fn test_nfa_overrides_bounded_backtracking() {
    let re = "example regex";
    let exec_builder = ExecBuilder::new(re).bounded_backtracking();
    
    // After calling nfa, it should still be Nfa(PikeVM)
    let updated_builder = exec_builder.nfa();
    assert_eq!(updated_builder.match_type, Some(MatchType::Nfa(MatchNfaType::PikeVM)));
}

#[test]
fn test_nfa_does_not_change_other_options() {
    let re = "example regex";
    let exec_builder = ExecBuilder::new(re).bytes(true).only_utf8(false);
    
    // Call nfa and check that other options remain unchanged
    let updated_builder = exec_builder.nfa();
    assert!(updated_builder.bytes);
    assert!(!updated_builder.only_utf8);
}

