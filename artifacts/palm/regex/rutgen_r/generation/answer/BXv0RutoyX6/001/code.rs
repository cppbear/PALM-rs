// Answer 0

#[test]
fn test_fmt() {
    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "Position(o: {:?}, l: {:?}, c: {:?})",
                self.offset, self.line, self.column
            )
        }
    }

    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 10, line: 2, column: 5 };
    let pos3 = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    
    assert_eq!(format!("{}", pos1), "Position(o: 0, l: 1, c: 1)");
    assert_eq!(format!("{}", pos2), "Position(o: 10, l: 2, c: 5)");
    assert_eq!(format!("{}", pos3), format!("Position(o: {}, l: {}, c: {})", usize::MAX, usize::MAX, usize::MAX));
}

