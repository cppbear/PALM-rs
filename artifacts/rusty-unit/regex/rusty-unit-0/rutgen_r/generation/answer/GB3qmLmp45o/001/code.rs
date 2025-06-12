// Answer 0

#[derive(Debug)]
struct InputAt {
    pos: usize,
    c: char,
    byte: Option<u8>,
    len: usize,
}

fn decode_utf8(slice: &[u8]) -> Result<(char, usize), ()> {
    // This is a simplified version of decode_utf8 for the sake of testing
    std::str::from_utf8(slice).ok().and_then(|s| s.chars().next()).map_or(Err(()), |c| {
        let bytes = c.len_utf8();
        Ok((c, bytes))
    })
}

struct Input {
    data: Vec<u8>,
}

impl Input {
    pub fn at(&self, i: usize) -> InputAt {
        let c = decode_utf8(&self.data[i..]).map(|(c, _)| c).unwrap();
        InputAt {
            pos: i,
            c: c,
            byte: None,
            len: c.len_utf8(),
        }
    }
}

#[test]
fn test_at_valid_character() {
    let input = Input {
        data: "Hello, World!".as_bytes().to_vec(),
    };
    let result = input.at(7);
    assert_eq!(result.pos, 7);
    assert_eq!(result.c, 'W');
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_first_character() {
    let input = Input {
        data: "你好".as_bytes().to_vec(),
    };
    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.c, '你');
    assert_eq!(result.len, 3);
}

#[test]
fn test_at_last_character() {
    let input = Input {
        data: "こんにちは".as_bytes().to_vec(),
    };
    let result = input.at(8);
    assert_eq!(result.pos, 8);
    assert_eq!(result.c, 'は');
    assert_eq!(result.len, 3);
}

#[should_panic]
#[test]
fn test_at_out_of_bounds() {
    let input = Input {
        data: "Hello!".as_bytes().to_vec(),
    };
    let _ = input.at(10); // This should panic
} 

#[test]
fn test_at_empty_input() {
    let input = Input {
        data: Vec::new(),
    };
    #[should_panic]
    let _ = input.at(0); // This should panic
}

