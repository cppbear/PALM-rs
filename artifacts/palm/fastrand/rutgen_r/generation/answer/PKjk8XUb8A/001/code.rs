// Answer 0

#[test]
fn test_uppercase_random_char() {
    struct RandomGenerator;

    impl RandomGenerator {
        pub fn choice(&self, chars: &[u8]) -> Option<&u8> {
            if chars.is_empty() {
                return None;
            }
            let index = 0; // always return first char for test purposes
            Some(&chars[index])
        }

        pub fn uppercase(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = RandomGenerator;

    assert_eq!(rng.uppercase(), 'A');
}

#[test]
fn test_uppercase_random_char_empty_choice() {
    struct EmptyRandomGenerator;

    impl EmptyRandomGenerator {
        pub fn choice(&self, _: &[u8]) -> Option<&u8> {
            None // Simulate an empty choice
        }

        pub fn uppercase(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = EmptyRandomGenerator;

    #[should_panic]
    let _ = rng.uppercase(); // This should panic due to unwrap on None
}

#[test]
fn test_uppercase_random_char_full_range() {
    struct FullRandomGenerator;

    impl FullRandomGenerator {
        pub fn choice(&self, chars: &[u8]) -> Option<&u8> {
            let index = 25; // Always return last char for test purposes
            Some(&chars[index])
        }

        pub fn uppercase(&mut self) -> char {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            *self.choice(CHARS).unwrap() as char
        }
    }

    let mut rng = FullRandomGenerator;

    assert_eq!(rng.uppercase(), 'Z');
}

