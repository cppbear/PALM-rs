// Answer 0

#[derive(Default)]
struct TestIndexMap {
    data: Vec<i32>,
}

impl TestIndexMap {
    fn borrow_mut(&mut self) -> &mut Self {
        self
    }

    fn move_index(&mut self, from: usize, to: usize) {
        if from >= self.data.len() || to >= self.data.len() {
            panic!("Index out of bounds");
        }
        if from == to {
            return; // No movement needed
        }
        let value = self.data.remove(from);
        self.data.insert(to, value);
    }
}

#[test]
fn test_move_index_valid_range() {
    let mut index_map = TestIndexMap { data: vec![1, 2, 3, 4, 5] };
    index_map.move_index(1, 3);
    assert_eq!(index_map.data, vec![1, 3, 4, 2, 5]);
}

#[test]
fn test_move_index_to_same_position() {
    let mut index_map = TestIndexMap { data: vec![1, 2, 3, 4, 5] };
    index_map.move_index(2, 2);
    assert_eq!(index_map.data, vec![1, 2, 3, 4, 5]);
}

#[should_panic(expected = "Index out of bounds")]
#[test]
fn test_move_index_from_out_of_bounds() {
    let mut index_map = TestIndexMap { data: vec![1, 2, 3, 4, 5] };
    index_map.move_index(5, 3);
}

#[should_panic(expected = "Index out of bounds")]
#[test]
fn test_move_index_to_out_of_bounds() {
    let mut index_map = TestIndexMap { data: vec![1, 2, 3, 4, 5] };
    index_map.move_index(1, 5);
}

#[should_panic(expected = "Index out of bounds")]
#[test]
fn test_move_index_both_out_of_bounds() {
    let mut index_map = TestIndexMap { data: vec![1, 2, 3, 4, 5] };
    index_map.move_index(6, 7);
}

