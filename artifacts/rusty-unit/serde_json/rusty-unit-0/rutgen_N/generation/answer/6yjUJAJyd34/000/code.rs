// Answer 0

#[derive(Debug, PartialEq)]
struct Position {
    line: usize,
    column: usize,
}

struct TestStruct {
    index: usize,
    positions: Vec<Position>,
}

impl TestStruct {
    fn position_of_index(&self, index: usize) -> Position {
        self.positions[index].clone()
    }

    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }
}

#[test]
fn test_position_valid_index() {
    let positions = vec![
        Position { line: 1, column: 1 },
        Position { line: 2, column: 5 },
        Position { line: 3, column: 10 },
    ];
    let test_struct = TestStruct { index: 1, positions };

    assert_eq!(test_struct.position(), Position { line: 2, column: 5 });
}

#[test]
fn test_position_zero_index() {
    let positions = vec![
        Position { line: 1, column: 1 },
        Position { line: 2, column: 5 },
    ];
    let test_struct = TestStruct { index: 0, positions };

    assert_eq!(test_struct.position(), Position { line: 1, column: 1 });
}

#[should_panic]
#[test]
fn test_position_out_of_bounds() {
    let positions = vec![
        Position { line: 1, column: 1 },
        Position { line: 2, column: 5 },
    ];
    let test_struct = TestStruct { index: 2, positions }; // Out of bounds access

    test_struct.position();
}

