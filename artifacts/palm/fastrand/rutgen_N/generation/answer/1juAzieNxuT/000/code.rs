// Answer 0

#[test]
fn test_lowercase_generates_valid_char() {
    struct RandomGenerator;

    impl RandomGenerator {
        fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[0]) // For test purposes, always return the first character
        }
        
        fn lowercase(&mut self) -> char {
            const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = RandomGenerator;
    let result = rng.lowercase();
    assert!(result.is_ascii_lowercase());
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_lowercase_boundary_cases() {
    struct RandomGenerator;

    impl RandomGenerator {
        fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[25]) // For test purposes, return the last character
        }
        
        fn lowercase(&mut self) -> char {
            const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = RandomGenerator;
    let result = rng.lowercase();
    assert_eq!(result, 'z');
}

#[test]
fn test_lowercase_first_case() {
    struct RandomGenerator;

    impl RandomGenerator {
        fn choice(&self, chars: &[u8]) -> Option<&u8> {
            Some(&chars[0]) // For test purposes, return the first character
        }
        
        fn lowercase(&mut self) -> char {
            const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = RandomGenerator;
    let result = rng.lowercase();
    assert_eq!(result, 'a');
}

