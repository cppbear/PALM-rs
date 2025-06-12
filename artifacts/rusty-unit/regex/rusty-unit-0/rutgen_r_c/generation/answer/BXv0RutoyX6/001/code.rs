// Answer 0

#[test]
fn test_position_debug_fmt() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", position);
    assert!(result.is_ok());
    assert_eq!(output, "Position(o: 0, l: 1, c: 1)");

    let position = Position { offset: 10, line: 2, column: 5 };
    output.clear();
    let result = write!(&mut output, "{:?}", position);
    assert!(result.is_ok());
    assert_eq!(output, "Position(o: 10, l: 2, c: 5)");

    let position = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    output.clear();
    let result = write!(&mut output, "{:?}", position);
    assert!(result.is_ok());
    assert_eq!(output, "Position(o: 18446744073709551615, l: 18446744073709551615, c: 18446744073709551615)");
}

