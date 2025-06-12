// Answer 0

#[test]
fn test_find_with_exact_length_haystack_and_non_empty_pat() {
    struct Regex {
        pat: Box<[u8]>,
        rare1i: usize,
        rare1: u8,
        rare2i: usize,
        rare2: u8,
    }

    impl Regex {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            let pat = &*self.pat;
            if haystack.len() < pat.len() || pat.is_empty() {
                return None;
            }
            let mut i = self.rare1i;
            while i < haystack.len() {
                i += match memchr::memchr(self.rare1, &haystack[i..]) {
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

    let regex = Regex {
        pat: Box::from(b"pattern"),
        rare1i: 0,
        rare1: b'p',
        rare2i: 6,
        rare2: b'n',
    };
    let haystack = b"pattern";

    let result = regex.find(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_with_empty_pat() {
    struct Regex {
        pat: Box<[u8]>,
        rare1i: usize,
        rare1: u8,
        rare2i: usize,
        rare2: u8,
    }

    impl Regex {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            let pat = &*self.pat;
            if haystack.len() < pat.len() || pat.is_empty() {
                return None;
            }
            let mut i = self.rare1i;
            while i < haystack.len() {
                i += match memchr::memchr(self.rare1, &haystack[i..]) {
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

    let regex = Regex {
        pat: Box::new([]), // empty pattern
        rare1i: 0,
        rare1: b'x',
        rare2i: 0,
        rare2: b'y',
    };
    let haystack = b"xyz";

    let result = regex.find(haystack);
    assert_eq!(result, None);
}

