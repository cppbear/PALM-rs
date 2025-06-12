// Answer 0

#[test]
fn test_as_str_valid() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    impl Alphabet {
        pub fn as_str(&self) -> &str {
            core::str::from_utf8(&self.symbols).unwrap()
        }
    }

    let alphabet = Alphabet {
        symbols: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec(),
    };

    assert_eq!(alphabet.as_str(), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    impl Alphabet {
        pub fn as_str(&self) -> &str {
            core::str::from_utf8(&self.symbols).unwrap()
        }
    }

    let alphabet = Alphabet {
        symbols: vec![0, 159, 146, 150], // Invalid UTF-8 sequence
    };

    alphabet.as_str();
}

