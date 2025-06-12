// Answer 0

#[test]
fn test_is_suffix_with_short_text() {
    struct SuffixPattern {
        pat: Vec<u8>,
    }

    impl SuffixPattern {
        fn len(&self) -> usize {
            self.pat.len()
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            if text.len() < self.len() {
                return false;
            }
            text[text.len() - self.len()..] == *self.pat
        }
    }

    let pattern = SuffixPattern { pat: b"hello".to_vec() };
    let short_text = b"hi";

    assert_eq!(pattern.is_suffix(short_text), false);
}

#[test]
fn test_is_suffix_with_exact_length() {
    struct SuffixPattern {
        pat: Vec<u8>,
    }

    impl SuffixPattern {
        fn len(&self) -> usize {
            self.pat.len()
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            if text.len() < self.len() {
                return false;
            }
            text[text.len() - self.len()..] == *self.pat
        }
    }

    let pattern = SuffixPattern { pat: b"test".to_vec() };
    let exact_length_text = b"123"; // length is less than the pattern

    assert_eq!(pattern.is_suffix(exact_length_text), false);
}

