// Answer 0

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
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
    let test_struct = TestStruct {
        index: 1,
        positions: vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 2 },
        ],
    };
    
    let pos = test_struct.position();
    assert_eq!(pos, Position { x: 1, y: 1 });
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_position_index_out_of_bounds_high() {
    let test_struct = TestStruct {
        index: 3,
        positions: vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 2 },
        ],
    };
    
    let _ = test_struct.position();
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_position_index_out_of_bounds_low() {
    let test_struct = TestStruct {
        index: usize::MAX,
        positions: vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 2, y: 2 },
        ],
    };
    
    let _ = test_struct.position();
}

