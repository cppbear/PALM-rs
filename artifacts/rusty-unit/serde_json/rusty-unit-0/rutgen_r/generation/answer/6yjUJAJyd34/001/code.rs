// Answer 0

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct TestStruct {
    index: usize,
    positions: Vec<Position>,
}

impl TestStruct {
    fn position_of_index(&self, index: usize) -> Position {
        if index >= self.positions.len() {
            panic!("Index out of bounds");
        }
        self.positions[index].clone()
    }

    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }
}

#[test]
fn test_position_valid_index() {
    let test_str = TestStruct {
        index: 1,
        positions: vec![Position { x: 0, y: 0 }, Position { x: 1, y: 1 }],
    };
    let pos = test_str.position();
    assert_eq!(pos.x, 1);
    assert_eq!(pos.y, 1);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_position_index_too_high() {
    let test_str = TestStruct {
        index: 2,
        positions: vec![Position { x: 0, y: 0 }, Position { x: 1, y: 1 }],
    };
    let _ = test_str.position();
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_position_index_equal_to_length() {
    let test_str = TestStruct {
        index: 2,
        positions: vec![Position { x: 0, y: 0 }],
    };
    let _ = test_str.position();
}

#[test]
fn test_position_zero_index() {
    let test_str = TestStruct {
        index: 0,
        positions: vec![Position { x: 2, y: 3 }, Position { x: 4, y: 5 }],
    };
    let pos = test_str.position();
    assert_eq!(pos.x, 2);
    assert_eq!(pos.y, 3);
}

