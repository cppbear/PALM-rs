// Answer 0

#[test]
fn test_cut_function() {
    // Test cutting an empty literal
    let mut literal_empty = Literal::empty();
    literal_empty.cut();
    assert!(literal_empty.is_cut());

    // Test cutting a non-empty literal
    let mut literal_non_empty = Literal::new(vec![b'a', b'b', b'c']);
    literal_non_empty.cut();
    assert!(literal_non_empty.is_cut());

    // Edge case: Test cutting a literal that is already cut
    let mut literal_already_cut = Literal::new(vec![b'x']);
    literal_already_cut.cut();
    literal_already_cut.cut(); // should not panic
    assert!(literal_already_cut.is_cut());
}

