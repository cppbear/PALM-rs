// Answer 0

#[derive(Debug, PartialEq)]
struct FreqyPacked {
    pat: Vec<u8>,
    char_len: usize,
    rare1: u8,
    rare1i: usize,
    rare2: u8,
    rare2i: usize,
}

impl FreqyPacked {
    fn empty() -> Self {
        FreqyPacked {
            pat: Vec::new(),
            char_len: 0,
            rare1: 0,
            rare1i: 0,
            rare2: 0,
            rare2i: 0,
        }
    }
}

fn freq_rank(b: u8) -> usize {
    // Placeholder implementation for the sake of the test
    b as usize
}

fn char_len_lossy(pat: &Vec<u8>) -> usize {
    // Placeholder implementation for the sake of the test
    pat.len()
}

fn new(pat: Vec<u8>) -> FreqyPacked {
    if pat.is_empty() {
        return FreqyPacked::empty();
    }

    let mut rare1 = pat[0];
    let mut rare2 = pat[0];
    for b in pat[1..].iter().cloned() {
        if freq_rank(b) < freq_rank(rare1) {
            rare1 = b;
        }
    }
    for &b in &pat {
        if rare1 == rare2 {
            rare2 = b
        } else if b != rare1 && freq_rank(b) < freq_rank(rare2) {
            rare2 = b;
        }
    }

    let rare1i = pat.iter().rposition(|&b| b == rare1).unwrap();
    let rare2i = pat.iter().rposition(|&b| b == rare2).unwrap();

    let char_len = char_len_lossy(&pat);
    FreqyPacked {
        pat: pat,
        char_len: char_len,
        rare1: rare1,
        rare1i: rare1i,
        rare2: rare2,
        rare2i: rare2i,
    }
}

#[test]
fn test_new_with_various_inputs() {
    // Test for a non-empty pattern
    let input = vec![5, 3, 1, 2, 3, 5];
    let result = new(input.clone());
    assert_eq!(result.pat, input);
    assert_eq!(result.rare1, 1);
    assert_eq!(result.rare2, 2);
    assert_eq!(result.rare1i, 2);
    assert_eq!(result.rare2i, 3);
    assert_eq!(result.char_len, 6);
}

#[test]
fn test_new_with_distinct_bytes() {
    // Test for a pattern with distinct bytes
    let input = vec![7, 8, 6, 5, 2, 3, 4];
    let result = new(input.clone());
    assert_eq!(result.pat, input);
    assert_eq!(result.rare1, 2);
    assert_eq!(result.rare2, 3);
    assert_eq!(result.rare1i, 4);
    assert_eq!(result.rare2i, 5);
    assert_eq!(result.char_len, 7);
}

#[test]
fn test_new_with_all_same_bytes() {
    // Test for a pattern with the same byte repeated
    let input = vec![9, 9, 9, 9];
    let result = new(input.clone());
    assert_eq!(result.pat, input);
    assert_eq!(result.rare1, 9);
    assert_eq!(result.rare2, 9);
    assert_eq!(result.rare1i, 3);
    assert_eq!(result.rare2i, 3);
    assert_eq!(result.char_len, 4);
}

