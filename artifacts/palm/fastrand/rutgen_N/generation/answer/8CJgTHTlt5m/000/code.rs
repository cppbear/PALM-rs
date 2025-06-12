// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use fastrand::Rng;

    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for TestIterator {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    #[test]
    fn test_choice_non_empty_iterator() {
        let iter = TestIterator::new(vec![1, 2, 3, 4, 5]);
        let result = choice(iter);
        assert!(result.is_some());
    }

    #[test]
    fn test_choice_empty_iterator() {
        let iter = TestIterator::new(vec![]);
        let result = choice(iter);
        assert!(result.is_none());
    }

    #[test]
    fn test_choice_exact_size() {
        let iter = TestIterator::new(vec![10, 20, 30]);
        let result = choice(iter);
        assert!(result.is_some());
        assert!(matches!(result, Some(10) | Some(20) | Some(30)));
    }

    #[test]
    fn test_choice_boundary_condition() {
        let iter = TestIterator::new(vec![5]);
        let result = choice(iter);
        assert_eq!(result, Some(5));
    }
}

