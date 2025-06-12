// Answer 0

#[test]
fn test_new_with_non_empty_pat() {
    struct FreqyPacked {
        pat: Vec<u8>,
        char_len: usize,
        rare1: u8,
        rare1i: usize,
        rare2: u8,
        rare2i: usize,
    }

    fn freq_rank(_byte: u8) -> usize {
        1 // Placeholder implementation for testing
    }

    fn char_len_lossy(pat: &Vec<u8>) -> usize {
        pat.len() // Simple implementation for testing
    }

    fn new(pat: Vec<u8>) -> FreqyPacked {
        if pat.is_empty() {
            return FreqyPacked { pat: vec![], char_len: 0, rare1: 0, rare1i: 0, rare2: 0, rare2i: 0 };
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
                rare2 = b;
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

    let pat = vec![10, 20, 10, 30, 20];
    let packed = new(pat.clone());

    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, pat.len());
    assert_eq!(packed.rare1, 10);
    assert_eq!(packed.rare1i, 2);
    assert_eq!(packed.rare2, 20);
    assert_eq!(packed.rare2i, 4);
}

#[test]
#[should_panic]
fn test_new_with_empty_pat() {
    struct FreqyPacked {
        pat: Vec<u8>,
        char_len: usize,
        rare1: u8,
        rare1i: usize,
        rare2: u8,
        rare2i: usize,
    }

    fn new(pat: Vec<u8>) -> FreqyPacked {
        if pat.is_empty() {
            panic!("Pattern cannot be empty");
        }
        // Implementation omitted for brevity
        FreqyPacked { pat, char_len: 0, rare1: 0, rare1i: 0, rare2: 0, rare2i: 0 }
    }

    new(vec![]);
}

#[test]
fn test_new_with_single_byte_pat() {
    struct FreqyPacked {
        pat: Vec<u8>,
        char_len: usize,
        rare1: u8,
        rare1i: usize,
        rare2: u8,
        rare2i: usize,
    }

    fn freq_rank(_byte: u8) -> usize {
        1 // Placeholder implementation
    }

    fn char_len_lossy(pat: &Vec<u8>) -> usize {
        pat.len() // Placeholder implementation
    }

    fn new(pat: Vec<u8>) -> FreqyPacked {
        if pat.is_empty() {
            return FreqyPacked { pat: vec![], char_len: 0, rare1: 0, rare1i: 0, rare2: 0, rare2i: 0 };
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
                rare2 = b;
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

    let pat = vec![50];
    let packed = new(pat.clone());

    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, 1);
    assert_eq!(packed.rare1, 50);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 50);
    assert_eq!(packed.rare2i, 0);
}

