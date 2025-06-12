// Answer 0

#[test]
fn test_find_end_exact_match() {
    struct RegexMatcher {
        literals: Vec<Vec<u8>>,
    }

    impl RegexMatcher {
        pub fn iter(&self) -> std::slice::Iter<Vec<u8>> {
            self.literals.iter()
        }

        pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[haystack.len() - lit.len()..] {
                    return Some((haystack.len() - lit.len(), haystack.len()));
                }
            }
            None
        }
    }

    let matcher = RegexMatcher {
        literals: vec![b"test".to_vec()],
    };
    let haystack = b"this is a test";
    let result = matcher.find_end(haystack);
    assert_eq!(result, Some((10, 14)));
}

#[test]
fn test_find_end_empty_haystack() {
    struct RegexMatcher {
        literals: Vec<Vec<u8>>,
    }

    impl RegexMatcher {
        pub fn iter(&self) -> std::slice::Iter<Vec<u8>> {
            self.literals.iter()
        }

        pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[haystack.len() - lit.len()..] {
                    return Some((haystack.len() - lit.len(), haystack.len()));
                }
            }
            None
        }
    }

    let matcher = RegexMatcher {
        literals: vec![b"test".to_vec()],
    };
    let haystack = b"";
    let result = matcher.find_end(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_end_multiple_literals() {
    struct RegexMatcher {
        literals: Vec<Vec<u8>>,
    }

    impl RegexMatcher {
        pub fn iter(&self) -> std::slice::Iter<Vec<u8>> {
            self.literals.iter()
        }

        pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[haystack.len() - lit.len()..] {
                    return Some((haystack.len() - lit.len(), haystack.len()));
                }
            }
            None
        }
    }

    let matcher = RegexMatcher {
        literals: vec![b"example".to_vec(), b"test".to_vec(), b"haystack".to_vec()],
    };
    let haystack = b"this is a test example";
    let result = matcher.find_end(haystack);
    assert_eq!(result, Some((15, 22)));
}

