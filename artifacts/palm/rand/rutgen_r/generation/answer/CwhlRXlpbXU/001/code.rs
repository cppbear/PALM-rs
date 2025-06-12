// Answer 0

#[test]
fn test_append_string_normal_case() {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};
    
    struct CharDistribution;

    impl Distribution<char> for CharDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            let chars = "abcdefghijklmnopqrstuvwxyz";
            let uniform = Uniform::from(0..chars.len());
            chars.chars().nth(uniform.sample(rng)).unwrap()
        }

        fn sample_iter<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn Iterator<Item = char>> {
            Box::new(std::iter::repeat_with(move || self.sample(rng)))
        }
    }

    let mut rng = rand::thread_rng();
    let mut output = String::new();
    let distribution = CharDistribution;
    
    distribution.append_string(&mut rng, &mut output, 5);
    
    assert_eq!(output.len(), 5);
}

#[test]
fn test_append_string_zero_length() {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};
    
    struct CharDistribution;

    impl Distribution<char> for CharDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            let chars = "abcdefghijklmnopqrstuvwxyz";
            let uniform = Uniform::from(0..chars.len());
            chars.chars().nth(uniform.sample(rng)).unwrap()
        }

        fn sample_iter<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn Iterator<Item = char>> {
            Box::new(std::iter::repeat_with(move || self.sample(rng)))
        }
    }

    let mut rng = rand::thread_rng();
    let mut output = String::new();
    let distribution = CharDistribution;

    distribution.append_string(&mut rng, &mut output, 0);

    assert_eq!(output.len(), 0);
}

#[test]
fn test_append_string_large_length() {
    use rand::Rng;
    use rand::distributions::{Distribution, Uniform};

    struct CharDistribution;

    impl Distribution<char> for CharDistribution {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            let chars = "abcdefghijklmnopqrstuvwxyz";
            let uniform = Uniform::from(0..chars.len());
            chars.chars().nth(uniform.sample(rng)).unwrap()
        }

        fn sample_iter<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn Iterator<Item = char>> {
            Box::new(std::iter::repeat_with(move || self.sample(rng)))
        }
    }

    let mut rng = rand::thread_rng();
    let mut output = String::new();
    let distribution = CharDistribution;
    
    distribution.append_string(&mut rng, &mut output, 1000);
    
    assert_eq!(output.len(), 1000);
}

