// Answer 0

#[test]
fn test_shortest_match_at_valid_start() {
    struct Matcher(Vec<u8>);

    impl Matcher {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start >= text.len() {
                return None;
            }
            let text_slice = &text[start..];
            self.0.iter().position(|&m| text_slice.starts_with(&[m])).map(|pos| pos + start)
        }
    }

    let matcher = Matcher(vec![b'a', b'b', b'c']);
    
    let result = matcher.shortest_match_at("abcabc", 0);
    assert_eq!(result, Some(0));

    let result = matcher.shortest_match_at("xyzabc", 3);
    assert_eq!(result, Some(3));

    let result = matcher.shortest_match_at("def", 0);
    assert_eq!(result, None);
}

#[test]
fn test_shortest_match_at_out_of_bounds_start() {
    struct Matcher(Vec<u8>);

    impl Matcher {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start >= text.len() {
                return None;
            }
            let text_slice = &text[start..];
            self.0.iter().position(|&m| text_slice.starts_with(&[m])).map(|pos| pos + start)
        }
    }

    let matcher = Matcher(vec![b'a', b'b', b'c']);
    
    let result = matcher.shortest_match_at("test", 10);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_shortest_match_at_negative_start() {
    struct Matcher(Vec<u8>);

    impl Matcher {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start >= text.len() {
                return None;
            }
            let text_slice = &text[start..];
            self.0.iter().position(|&m| text_slice.starts_with(&[m])).map(|pos| pos + start)
        }
    }

    let matcher = Matcher(vec![b'a', b'b', b'c']);
    
    // This should panic since the start is negative (handled implicitly by integer types in Rust)
    let _result = matcher.shortest_match_at("example", usize::MAX);
}

