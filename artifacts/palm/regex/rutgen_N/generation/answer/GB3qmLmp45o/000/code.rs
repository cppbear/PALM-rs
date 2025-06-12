// Answer 0

#[derive(Debug)]
struct InputAt {
    pos: usize,
    c: char,
    byte: Option<u8>,
    len: usize,
}

struct Input {
    data: String,
}

impl Input {
    fn at(&self, i: usize) -> InputAt {
        let c = self.data.chars().nth(i).expect("Index out of bounds");
        InputAt {
            pos: i,
            c: c,
            byte: None,
            len: c.len_utf8(),
        }
    }
}

#[test]
fn test_at_valid_index() {
    let input = Input { data: "hello".to_string() };
    let result = input.at(1);
    assert_eq!(result.pos, 1);
    assert_eq!(result.c, 'e');
    assert_eq!(result.len, 'e'.len_utf8());
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_at_index_out_of_bounds() {
    let input = Input { data: "hello".to_string() };
    let _result = input.at(10);
}

