// Answer 0

#[test]
fn test_bounded_backtracking() {
    // Arrange
    let initial_regex = "a*";
    let exec_builder = ExecBuilder::new(initial_regex);

    // Act
    let exec_builder_backtrack = exec_builder.bounded_backtracking();

    // Assert
    assert_eq!(exec_builder_backtrack.match_type, Some(MatchType::Nfa(MatchNfaType::Backtrack)));
}

#[test]
fn test_bounded_backtracking_with_multiple_calls() {
    // Arrange
    let initial_regex = "b+";
    let exec_builder = ExecBuilder::new(initial_regex)
        .automatic(); // Set to automatic first

    // Act
    let exec_builder_backtrack = exec_builder.bounded_backtracking();

    // Assert
    assert_eq!(exec_builder_backtrack.match_type, Some(MatchType::Nfa(MatchNfaType::Backtrack)));
}

#[test]
fn test_bounded_backtracking_overrides_nfa() {
    // Arrange
    let initial_regex = "c{2,5}";
    let exec_builder = ExecBuilder::new(initial_regex)
        .nfa(); // Set to NFA first

    // Act
    let exec_builder_backtrack = exec_builder.bounded_backtracking();

    // Assert
    assert_eq!(exec_builder_backtrack.match_type, Some(MatchType::Nfa(MatchNfaType::Backtrack)));
}

