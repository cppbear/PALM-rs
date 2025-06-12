// Answer 0

#[test]
fn test_as_bytes() {
    struct TextWrapper {
        text: Vec<u8>,
    }

    impl TextWrapper {
        fn as_bytes(&self) -> &[u8] {
            &self.text
        }
    }

    let text_wrapper = TextWrapper {
        text: b"Hello, world!".to_vec(),
    };

    assert_eq!(text_wrapper.as_bytes(), b"Hello, world!");
}

#[test]
fn test_as_bytes_empty() {
    struct TextWrapper {
        text: Vec<u8>,
    }

    impl TextWrapper {
        fn as_bytes(&self) -> &[u8] {
            &self.text
        }
    }

    let text_wrapper = TextWrapper {
        text: Vec::new(),
    };

    assert_eq!(text_wrapper.as_bytes(), b"");
}

