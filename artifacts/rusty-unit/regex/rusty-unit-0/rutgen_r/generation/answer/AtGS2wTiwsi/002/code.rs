// Answer 0

#[test]
fn test_find_start_matches_full_haystack() {
    struct Literal<'a> {
        value: &'a [u8],
    }

    impl<'a> Iterator for Literal<'a> {
        type Item = &'a [u8];

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.value;
            self.value = &[]; // Consume the iterator after first call
            Some(result)
        }
    }

    struct Matcher<'a> {
        literals: Vec<&'a [u8]>,
    }

    impl<'a> Matcher<'a> {
        fn iter(&self) -> Literal {
            Literal {
                value: self.literals.get(0).unwrap_or(&&[]),
            }
        }

        fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[0..lit.len()] {
                    return Some((0, lit.len()));
                }
            }
            None
        }
    }

    let matcher = Matcher {
        literals: vec![b"test"],
    };

    let result = matcher.find_start(b"test");
    assert_eq!(result, Some((0, 4)));
}

#[test]
fn test_find_start_no_match() {
    struct Literal<'a> {
        value: &'a [u8],
    }

    impl<'a> Iterator for Literal<'a> {
        type Item = &'a [u8];

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.value;
            self.value = &[]; // Consume the iterator after first call
            Some(result)
        }
    }

    struct Matcher<'a> {
        literals: Vec<&'a [u8]>,
    }

    impl<'a> Matcher<'a> {
        fn iter(&self) -> Literal {
            Literal {
                value: self.literals.get(0).unwrap_or(&&[]),
            }
        }

        fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[0..lit.len()] {
                    return Some((0, lit.len()));
                }
            }
            None
        }
    }

    let matcher = Matcher {
        literals: vec![b"example"],
    };

    let result = matcher.find_start(b"test");
    assert_eq!(result, None);
}

#[test]
fn test_find_start_empty_haystack() {
    struct Literal<'a> {
        value: &'a [u8],
    }

    impl<'a> Iterator for Literal<'a> {
        type Item = &'a [u8];

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.value;
            self.value = &[]; // Consume the iterator after first call
            Some(result)
        }
    }

    struct Matcher<'a> {
        literals: Vec<&'a [u8]>,
    }

    impl<'a> Matcher<'a> {
        fn iter(&self) -> Literal {
            Literal {
                value: self.literals.get(0).unwrap_or(&&[]),
            }
        }

        fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            for lit in self.iter() {
                if lit.len() > haystack.len() {
                    continue;
                }
                if lit == &haystack[0..lit.len()] {
                    return Some((0, lit.len()));
                }
            }
            None
        }
    }

    let matcher = Matcher {
        literals: vec![b""],
    };

    let result = matcher.find_start(b"");
    assert_eq!(result, Some((0, 0)));
}

