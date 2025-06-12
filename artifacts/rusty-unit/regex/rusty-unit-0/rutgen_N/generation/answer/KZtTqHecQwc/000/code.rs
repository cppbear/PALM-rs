// Answer 0

#[derive(Debug)]
struct InputAt {
    value: usize,
}

struct Input {
    data: Vec<usize>,
}

impl Input {
    fn at(&self, i: usize) -> InputAt {
        InputAt { value: self.data[i] }
    }
}

impl std::ops::Deref for Input {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[test]
fn test_at_with_valid_index() {
    let input = Input { data: vec![0, 1, 2, 3, 4] };
    let result = input.at(2);
    assert_eq!(result.value, 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_at_with_out_of_bounds_index() {
    let input = Input { data: vec![0, 1, 2] };
    let _ = input.at(5);
}

