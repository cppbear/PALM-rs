// Answer 0

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
    fn empty() -> FreqyPacked {
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

fn freq_rank(_: u8) -> usize {
    0 // Placeholder implementation for testing
}

fn char_len_lossy(_: &Vec<u8>) -> usize {
    0 // Placeholder implementation for testing
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
        pat,
        char_len,
        rare1,
        rare1i,
        rare2,
        rare2i,
    }
}

#[test]
fn test_new_with_valid_data() {
    let pattern = vec![1, 2, 3, 1, 4, 2]; // Valid non-empty pat
    let freqy = new(pattern);
    assert_eq!(freqy.pat.len(), 6);
    assert!(freqy.rare1 != freqy.rare2);
}

#[test]
#[should_panic]
fn test_new_with_empty_data() {
    let pattern: Vec<u8> = Vec::new(); // Should panic on empty pat
    new(pattern);
}

#[test]
fn test_new_with_repeated_elements() {
    let pattern = vec![5, 5, 3, 2, 2, 1]; 
    let freqy = new(pattern);
    assert_eq!(freqy.rare1, 1);
    assert_eq!(freqy.rare2, 2);
}

#[test]
fn test_new_with_all_identical_elements() {
    let pattern = vec![3, 3, 3, 3, 3]; 
    let freqy = new(pattern);
    assert_eq!(freqy.rare1, 3);
    assert_eq!(freqy.rare2, 3); // rare1 and rare2 are the same
}

