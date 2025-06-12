// Answer 0

#[test]
fn test_position_fmt_zero_values() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let _ = format!("{:?}", position);
}

#[test]
fn test_position_fmt_max_offset() {
    let position = Position { offset: usize::MAX, line: 1, column: 1 };
    let _ = format!("{:?}", position);
}

#[test]
fn test_position_fmt_max_line() {
    let position = Position { offset: 0, line: usize::MAX, column: 1 };
    let _ = format!("{:?}", position);
}

#[test]
fn test_position_fmt_max_column() {
    let position = Position { offset: 0, line: 1, column: usize::MAX };
    let _ = format!("{:?}", position);
}

#[test]
fn test_position_fmt_large_values() {
    let position = Position { offset: 123456789, line: 123456789, column: 123456789 };
    let _ = format!("{:?}", position);
}

