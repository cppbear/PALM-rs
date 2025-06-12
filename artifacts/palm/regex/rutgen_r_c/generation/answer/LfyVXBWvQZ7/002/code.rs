// Answer 0

#[test]
fn test_repetition_range_bounded_invalid() {
    // Given a RepetitionRange that is bounded with start greater than end
    let range = RepetitionRange::Bounded(3, 2);
    
    // When checking if the range is valid
    let result = range.is_valid();
    
    // Then the result should be false
    assert_eq!(result, false);
}

