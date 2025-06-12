// Answer 0

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    struct Distribution {
        num_choices: NonZeroUsize,
    }

    impl Distribution {
        pub fn new(num_choices: NonZeroUsize) -> Self {
            Self { num_choices }
        }

        pub fn num_choices(&self) -> NonZeroUsize {
            self.num_choices
        }
    }

    #[test]
    fn test_num_choices() {
        let num_choices = NonZeroUsize::new(5).unwrap();
        let distribution = Distribution::new(num_choices);
        assert_eq!(distribution.num_choices(), NonZeroUsize::new(5).unwrap());
    }

    #[test]
    fn test_num_choices_edge_case() {
        let num_choices = NonZeroUsize::new(1).unwrap();
        let distribution = Distribution::new(num_choices);
        assert_eq!(distribution.num_choices(), NonZeroUsize::new(1).unwrap());
    }
}

