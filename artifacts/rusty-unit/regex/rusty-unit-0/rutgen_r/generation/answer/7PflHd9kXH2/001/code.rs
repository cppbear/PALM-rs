// Answer 0

#[derive(Debug, PartialEq)]
struct FreqyPacked {
    pat: Vec<u8>,
    char_len: usize,
    rare1: usize,
    rare1i: usize,
    rare2: usize,
    rare2i: usize,
}

fn empty() -> FreqyPacked {
    FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    }
}

#[test]
fn test_empty_function() {
    let result = empty();
    let expected = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    
    assert_eq!(result, expected);
}

