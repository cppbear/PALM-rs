// Answer 0

#[test]
fn test_find_end_no_matches() {
    struct Literal {
        data: Vec<u8>,
    }

    impl Literal {
        fn iter(&self) -> std::slice::Iter<u8> {
            self.data.iter()
        }
    }

    let empty_literal = Literal { data: vec![] };
    let haystack: &[u8] = b"abcde";

    let result = empty_literal.find_end(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_end_lit_longer_than_haystack() {
    struct Literal {
        data: Vec<u8>,
    }

    impl Literal {
        fn iter(&self) -> std::slice::Iter<u8> {
            self.data.iter()
        }
    }

    let long_literal = Literal { data: b"abcdefg".to_vec() };
    let haystack: &[u8] = b"abcde";

    let result = long_literal.find_end(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_end_empty_haystack() {
    struct Literal {
        data: Vec<u8>,
    }

    impl Literal {
        fn iter(&self) -> std::slice::Iter<u8> {
            self.data.iter()
        }
    }

    let non_empty_literal = Literal { data: b"abc".to_vec() };
    let empty_haystack: &[u8] = b"";

    let result = non_empty_literal.find_end(empty_haystack);
    assert_eq!(result, None);
}

