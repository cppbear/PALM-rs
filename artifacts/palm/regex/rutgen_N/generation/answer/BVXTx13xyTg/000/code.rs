// Answer 0

#[test]
fn test_find_end_exact_match() {
    struct LiteralIterator<'a> {
        literals: &'a [&'a [u8]],
        index: usize,
    }

    impl<'a> LiteralIterator<'a> {
        fn new(literals: &'a [&'a [u8]]) -> Self {
            Self { literals, index: 0 }
        }

        fn iter(&mut self) -> Option<&'a [u8]> {
            if self.index < self.literals.len() {
                let result = Some(self.literals[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    impl LiteralIterator<'_> {
        pub fn find_end(&mut self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.literals {
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

    let literals: [&[u8]; 1] = [&b"test"[..]];
    let mut lit_iter = LiteralIterator::new(&literals);
    let result = lit_iter.find_end(b"this is a test");
    assert_eq!(result, Some((10, 14)));
}

#[test]
fn test_find_end_no_match() {
    struct LiteralIterator<'a> {
        literals: &'a [&'a [u8]],
        index: usize,
    }

    impl<'a> LiteralIterator<'a> {
        fn new(literals: &'a [&'a [u8]]) -> Self {
            Self { literals, index: 0 }
        }

        fn iter(&mut self) -> Option<&'a [u8]> {
            if self.index < self.literals.len() {
                let result = Some(self.literals[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    impl LiteralIterator<'_> {
        pub fn find_end(&mut self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.literals {
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

    let literals: [&[u8]; 1] = [&b"nope"[..]];
    let mut lit_iter = LiteralIterator::new(&literals);
    let result = lit_iter.find_end(b"this is a test");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_partial_match() {
    struct LiteralIterator<'a> {
        literals: &'a [&'a [u8]],
        index: usize,
    }

    impl<'a> LiteralIterator<'a> {
        fn new(literals: &'a [&'a [u8]]) -> Self {
            Self { literals, index: 0 }
        }

        fn iter(&mut self) -> Option<&'a [u8]> {
            if self.index < self.literals.len() {
                let result = Some(self.literals[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    impl LiteralIterator<'_> {
        pub fn find_end(&mut self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.literals {
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

    let literals: [&[u8]; 2] = [&b"test"[..], &b"ing"[..]];
    let mut lit_iter = LiteralIterator::new(&literals);
    let result = lit_iter.find_end(b"this is testing");
    assert_eq!(result, Some((10, 14)));
}

