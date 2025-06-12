// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct Bucket<T> {
        value: T,
    }

    struct Slice<T> {
        entries: Vec<Bucket<T>>,
    }

    impl<T> Slice<T> {
        pub fn last(&self) -> Option<&T> {
            self.entries.last().map(|bucket| &bucket.value)
        }
    }

    #[test]
    fn test_last_empty_slice() {
        let slice: Slice<i32> = Slice { entries: vec![] };
        assert_eq!(slice.last(), None);
    }

    #[test]
    fn test_last_non_empty_slice() {
        let slice = Slice {
            entries: vec![Bucket { value: 1 }, Bucket { value: 2 }, Bucket { value: 3 }],
        };
        assert_eq!(slice.last(), Some(&3));
    }

    #[test]
    fn test_last_single_element_slice() {
        let slice = Slice {
            entries: vec![Bucket { value: 42 }],
        };
        assert_eq!(slice.last(), Some(&42));
    }

    #[test]
    fn test_last_with_multiple_same_values() {
        let slice = Slice {
            entries: vec![Bucket { value: 5 }, Bucket { value: 5 }, Bucket { value: 5 }],
        };
        assert_eq!(slice.last(), Some(&5));
    }
}

