// Answer 0

#[derive(Debug, Clone)]
struct InputAt {
    pos: usize,
    c: Option<char>,
    byte: Option<u8>,
    len: usize,
}

struct TestInput {
    data: Vec<u8>,
}

impl TestInput {
    fn get(&self, i: usize) -> Option<&u8> {
        self.data.get(i)
    }

    fn at(&self, i: usize) -> InputAt {
        InputAt {
            pos: i,
            c: None.into(),
            byte: self.get(i).cloned(),
            len: 1,
        }
    }
}

#[test]
fn test_at_valid_index() {
    let input = TestInput { data: vec![97, 98, 99] }; // ASCII for 'a', 'b', 'c'
    let result = input.at(1);
    assert_eq!(result, InputAt { pos: 1, c: None, byte: Some(98), len: 1 });
}

#[test]
fn test_at_zero_index() {
    let input = TestInput { data: vec![100, 101] }; // ASCII for 'd', 'e'
    let result = input.at(0);
    assert_eq!(result, InputAt { pos: 0, c: None, byte: Some(100), len: 1 });
}

#[test]
fn test_at_out_of_bounds() {
    let input = TestInput { data: vec![120, 121, 122] }; // ASCII for 'x', 'y', 'z'
    let result = input.at(3); // Out of bounds index
    assert_eq!(result, InputAt { pos: 3, c: None, byte: None, len: 1 });
}

#[test]
fn test_at_empty_input() {
    let input = TestInput { data: vec![] };
    let result = input.at(0); // Attempting to access the first index of an empty input
    assert_eq!(result, InputAt { pos: 0, c: None, byte: None, len: 1 });
}

