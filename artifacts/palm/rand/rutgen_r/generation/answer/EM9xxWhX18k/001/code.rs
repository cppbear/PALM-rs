// Answer 0

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::distributions::Uniform;
    use std::marker::PhantomData;

    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn gen(&mut self) -> u32 {
            self.value
        }

        fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            // Ensure we generate values that may trigger panic in a controlled manner
            if self.value < range.start || self.value >= range.end {
                panic!("Generated value out of range");
            }
            self.value
        }

        // Other Rng trait methods might be required here for a complete implementation
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // Example implementation; does not need to be comprehensive
            for byte in dest.iter_mut() {
                *byte = self.value as u8;
            }
        }
    }

    #[test]
    fn test_sample_valid_char() {
        let mut rng = TestRng { value: 0xDFFF };
        let result = sample(&Uniform::new(0xDFFF, 0x11_0000).unwrap(), &mut rng);
        assert!(result >= '\u{0}' && result <= '\u{D7FF}' || result >= '\u{E000}' && result <= '\u{10FFFF}');
    }

    #[should_panic(expected = "Generated value out of range")]
    #[test]
    fn test_sample_panic_on_uniform_new() {
        let mut rng = TestRng { value: 0xDEAD }; // Value out of valid range
        let _ = sample(&Uniform::new(0xD800, 0x11_0000).unwrap(), &mut rng);
    }
}

