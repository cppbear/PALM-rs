// Answer 0

#[test]
fn test_find_haystack_shorter_than_pattern() {
    struct Pattern {
        pat: Vec<u8>,
        rare1: u8,
        rare1i: usize,
        rare2: u8,
        rare2i: usize,
    }

    impl Pattern {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            let pat = &*self.pat;
            if haystack.len() < pat.len() || pat.is_empty() {
                return None;
            }
            let mut i = self.rare1i;
            while i < haystack.len() {
                i += match memchr(self.rare1, &haystack[i..]) {
                    None => return None,
                    Some(i) => i,
                };
                let start = i - self.rare1i;
                let end = start + pat.len();
                if end > haystack.len() {
                    return None;
                }
                let aligned = &haystack[start..end];
                if aligned[self.rare2i] == self.rare2 && aligned == &*self.pat {
                    return Some(start);
                }
                i += 1;
            }
            None
        }
    }

    let pattern = Pattern {
        pat: vec![1, 2, 3],
        rare1: 4,
        rare1i: 0,
        rare2: 5,
        rare2i: 0,
    };

    let haystack = vec![4]; // haystack has length 1, pattern has length 3
    let result = pattern.find(&haystack);
    assert_eq!(result, None);
}

