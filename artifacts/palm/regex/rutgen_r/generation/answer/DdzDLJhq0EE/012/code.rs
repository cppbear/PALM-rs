// Answer 0

fn freq_rank(byte: u8) -> usize {
    // Placeholder implementation, replace with actual ranking logic
    byte as usize
}

fn char_len_lossy(pat: &Vec<u8>) -> usize {
    // Placeholder implementation, replace with actual length calculation logic
    pat.len()
}

#[derive(Debug)]
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
            pat: vec![],
            char_len: 0,
            rare1: 0,
            rare1i: 0,
            rare2: 0,
            rare2i: 0,
        }
    }
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
fn test_new_with_unique_bytes() {
    let pat = vec![3, 1, 2, 1, 4];
    let result = new(pat);
    assert_eq!(result.pat, vec![3, 1, 2, 1, 4]);
    assert_eq!(result.char_len, 5);
    assert_eq!(result.rare1, 1);
    assert_eq!(result.rare1i, 3);
    assert_eq!(result.rare2, 2);
    assert_eq!(result.rare2i, 2);
}

#[test]
fn test_new_with_repeating_bytes() {
    let pat = vec![5, 5, 1, 2, 1, 2];
    let result = new(pat);
    assert_eq!(result.pat, vec![5, 5, 1, 2, 1, 2]);
    assert_eq!(result.char_len, 6);
    assert_eq!(result.rare1, 1);
    assert_eq!(result.rare1i, 4);
    assert_eq!(result.rare2, 2);
    assert_eq!(result.rare2i, 3);
}

#[test]
fn test_new_with_identical_bytes() {
    let pat = vec![7, 7, 7, 7, 7];
    let result = new(pat);
    assert_eq!(result.pat, vec![7, 7, 7, 7, 7]);
    assert_eq!(result.char_len, 5);
    assert_eq!(result.rare1, 7);
    assert_eq!(result.rare1i, 4);
    assert_eq!(result.rare2, 7);
    assert_eq!(result.rare2i, 4);
}

#[should_panic]
#[test]
fn test_new_with_empty_vector() {
    let pat: Vec<u8> = vec![];
    let _result = new(pat);
}

