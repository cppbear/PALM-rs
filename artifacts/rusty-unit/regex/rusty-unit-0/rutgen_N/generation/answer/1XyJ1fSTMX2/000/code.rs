// Answer 0

#[derive(Clone)]
struct Input {
    data: Vec<u8>,
}

impl Input {
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

struct InputAt {
    pos: usize,
    c: Option<char>,
    byte: Option<u8>,
    len: usize,
}

#[test]
fn test_at_with_valid_index() {
    let input = Input { data: vec![1, 2, 3, 4, 5] };
    let result = input.at(2);
    assert_eq!(result.pos, 2);
    assert_eq!(result.byte, Some(3));
    assert_eq!(result.len, 1);
    assert_eq!(result.c, None);
}

#[test]
fn test_at_with_index_out_of_bounds() {
    let input = Input { data: vec![1, 2, 3, 4, 5] };
    let result = input.at(10);
    assert_eq!(result.pos, 10);
    assert_eq!(result.byte, None);
    assert_eq!(result.len, 1);
    assert_eq!(result.c, None);
}

#[test]
fn test_at_with_first_index() {
    let input = Input { data: vec![1, 2, 3, 4, 5] };
    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.byte, Some(1));
    assert_eq!(result.len, 1);
    assert_eq!(result.c, None);
}

#[test]
fn test_at_with_last_index() {
    let input = Input { data: vec![1, 2, 3, 4, 5] };
    let result = input.at(4);
    assert_eq!(result.pos, 4);
    assert_eq!(result.byte, Some(5));
    assert_eq!(result.len, 1);
    assert_eq!(result.c, None);
}

