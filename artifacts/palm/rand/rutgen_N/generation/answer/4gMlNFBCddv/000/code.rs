// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    struct DummyDistribution;

    impl DummyDistribution {
        fn sample_iter<R: Rng>(&self, rng: &mut R) -> impl Iterator<Item = u8> {
            std::iter::repeat_with(move || rng.gen_range(65..=90)) // ASCII A-Z
        }
    }

    #[test]
    fn test_append_string_empty() {
        let mut rng = rand::thread_rng();
        let mut my_string = String::new();
        let distribution = DummyDistribution;

        distribution.append_string(&mut rng, &mut my_string, 0);
        
        assert_eq!(my_string, "");
    }

    #[test]
    fn test_append_string_non_empty() {
        let mut rng = rand::thread_rng();
        let mut my_string = String::new();
        let distribution = DummyDistribution;

        distribution.append_string(&mut rng, &mut my_string, 5);
        
        assert_eq!(my_string.len(), 5);
        for byte in my_string.bytes() {
            assert!(byte >= 65 && byte <= 90); // Ensure we have ASCII A-Z
        }
    }

    #[test]
    fn test_append_string_large() {
        let mut rng = rand::thread_rng();
        let mut my_string = String::new();
        let distribution = DummyDistribution;

        distribution.append_string(&mut rng, &mut my_string, 100);
        
        assert_eq!(my_string.len(), 100);
        for byte in my_string.bytes() {
            assert!(byte >= 65 && byte <= 90); // Ensure we have ASCII A-Z
        }
    }
}

