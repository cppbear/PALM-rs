// Answer 0

#[test]
fn test_alphanumeric_generated_char() {
    struct MockRng;

    impl MockRng {
        pub fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[0]) // Always return the first character to avoid panic
        }
        
        pub fn alphanumeric(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = MockRng;
    let result = rng.alphanumeric();
    assert!(result.is_ascii_alphanumeric());
}

#[test]
fn test_alphanumeric_bounds() {
    struct MockRng;

    impl MockRng {
        pub fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[36]) // Choose the last character in the array to test bounds 
        }
        
        pub fn alphanumeric(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = MockRng;
    let result = rng.alphanumeric();
    assert_eq!(result, '9'); // Expecting the last character to be '9'
}

#[test]
#[should_panic]
fn test_alphanumeric_choice_panics() {
    struct MockRng;

    impl MockRng {
        pub fn choice(&self, _: &[u8]) -> Option<&u8> {
            None // Forcing a panic by returning None
        }
        
        pub fn alphanumeric(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = MockRng;
    rng.alphanumeric(); // This should panic
}

