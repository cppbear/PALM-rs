// Answer 0

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::distributions::{Distribution, Standard};
    use rand::seq::SliceRandom;

    struct CharDistribution;

    impl Distribution<char> for CharDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            let charset = "abcdefghijklmnopqrstuvwxyz";
            *charset.choose(rng).unwrap()
        }
    }

    #[test]
    fn test_append_string() {
        let mut rng = rand::thread_rng();
        let mut s = String::new();
        let len = 10;

        let distribution = CharDistribution;
        distribution.sample_iter(&mut rng).take(len).for_each(|c| s.push(c));

        assert_eq!(s.len(), len);
    }

    #[test]
    fn test_append_string_empty() {
        let mut rng = rand::thread_rng();
        let mut s = String::new();
        let len = 0;

        let distribution = CharDistribution;
        distribution.sample_iter(&mut rng).take(len).for_each(|c| s.push(c));

        assert_eq!(s.len(), len);
        assert!(s.is_empty());
    }
    
    #[test]
    fn test_append_string_large() {
        let mut rng = rand::thread_rng();
        let mut s = String::new();
        let len = 1000;

        let distribution = CharDistribution;
        distribution.sample_iter(&mut rng).take(len).for_each(|c| s.push(c));

        assert_eq!(s.len(), len);
    }
}

