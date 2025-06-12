// Answer 0

#[test]
fn test_find_with_haystack_equal_to_pat_length() {
    struct TestStruct {
        pat: Vec<u8>,
        rare1: u8,
        rare1i: usize,
        rare2: u8,
        rare2i: usize,
    }

    impl TestStruct {
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

    let test_struct = TestStruct {
        pat: b"abc".to_vec(),
        rare1: b'a'[0],
        rare1i: 0,
        rare2: b'c'[0],
        rare2i: 2,
    };
    
    let haystack = b"abc"; // haystack.len() == pat.len() == 3

    let result = test_struct.find(haystack);
    assert_eq!(result, None); // As we expect None when i == haystack.len()
}

