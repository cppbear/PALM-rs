fn new(pat: Vec<u8>) -> FreqyPacked {
        if pat.is_empty() {
            return FreqyPacked::empty();
        }

        // Find the rarest two bytes. Try to make them distinct (but it's not
        // required).
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

        // And find the offsets of their last occurrences.
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