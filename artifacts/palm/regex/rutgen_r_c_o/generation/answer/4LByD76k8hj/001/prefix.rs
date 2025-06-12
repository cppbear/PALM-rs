// Answer 0

#[test]
fn test_splat_with_minimum_values() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let _span = splat(pos);
}

#[test]
fn test_splat_with_mid_range_values() {
    let pos = Position { offset: 500, line: 5, column: 10 };
    let _span = splat(pos);
}

#[test]
fn test_splat_with_large_values() {
    let pos = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    let _span = splat(pos);
}

#[test]
fn test_splat_with_arbitrary_large_offset() {
    let pos = Position { offset: 10000, line: 100, column: 200 };
    let _span = splat(pos);
}

#[test]
fn test_splat_with_arbitrary_values() {
    let pos = Position { offset: 1024, line: 20, column: 30 };
    let _span = splat(pos);
}

