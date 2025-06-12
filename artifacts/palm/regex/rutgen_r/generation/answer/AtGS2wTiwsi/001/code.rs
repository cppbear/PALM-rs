// Answer 0

#[test]
fn test_find_start_with_empty_haystack() {
    struct Literal<'a> {
        data: &'a [&'a [u8]],
    }

    impl<'a> Literal<'a> {
        pub fn iter(&self) -> std::slice::Iter<'_, &'a [u8]> {
            self.data.iter()
        }

        pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
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

    let literals = Literal { data: &[] }; // constraint: lit in self.iter() is false
    let result = literals.find_start(b"");
    assert_eq!(result, None); // expected return: None
}

#[test]
fn test_find_start_with_large_literal() {
    struct Literal<'a> {
        data: &'a [&'a [u8]],
    }

    impl<'a> Literal<'a> {
        pub fn iter(&self) -> std::slice::Iter<'_, &'a [u8]> {
            self.data.iter()
        }

        pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
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

    let literals = Literal { data: &[b"longliteral"] }; // constraint: lit.len() > haystack.len()
    let result = literals.find_start(b"short");
    assert_eq!(result, None); // expected return: None
}

#[test]
fn test_find_start_with_non_matching_literal() {
    struct Literal<'a> {
        data: &'a [&'a [u8]],
    }

    impl<'a> Literal<'a> {
        pub fn iter(&self) -> std::slice::Iter<'_, &'a [u8]> {
            self.data.iter()
        }

        pub fn find_start(&self, haystack: &[u8]) -> Option<(usize, usize)> {
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

    let literals = Literal { data: &[b"notmatching"] }; // constraint: lit in self.iter() is true
    let result = literals.find_start(b"short");
    assert_eq!(result, None); // expected return: None
}

