// Answer 0

#[test]
fn test_deref_with_valid_text() {
    struct TextHolder {
        text: Vec<u8>,
    }

    impl TextHolder {
        fn new(text: Vec<u8>) -> Self {
            TextHolder { text }
        }

        fn deref(&self) -> &[u8] {
            self.text.as_slice()
        }
    }

    let holder = TextHolder::new(vec![97, 98, 99]); // ASCII for 'abc'
    let result = holder.deref();
    assert_eq!(result, b"abc");
}

#[test]
fn test_deref_with_empty_text() {
    struct TextHolder {
        text: Vec<u8>,
    }

    impl TextHolder {
        fn new(text: Vec<u8>) -> Self {
            TextHolder { text }
        }

        fn deref(&self) -> &[u8] {
            self.text.as_slice()
        }
    }

    let holder = TextHolder::new(vec![]);
    let result = holder.deref();
    assert_eq!(result, b"");
}

#[test]
fn test_deref_with_large_text() {
    struct TextHolder {
        text: Vec<u8>,
    }

    impl TextHolder {
        fn new(text: Vec<u8>) -> Self {
            TextHolder { text }
        }

        fn deref(&self) -> &[u8] {
            self.text.as_slice()
        }
    }

    let large_text = vec![120; 10_000]; // 10,000 bytes of `x`
    let holder = TextHolder::new(large_text);
    let result = holder.deref();
    assert_eq!(result.len(), 10_000);
    assert!(result.iter().all(|&byte| byte == 120));
}

