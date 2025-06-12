// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct Iter {
        size: usize,
    }

    impl Iter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, Some(self.size))
        }
    }

    struct Collection {
        iter: Iter,
    }

    impl Collection {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let collection = Collection { iter: Iter { size: 5 } };
    assert_eq!(collection.size_hint(), Some(5));
}

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct Iter {
        lower: usize,
        upper: usize,
    }

    impl Iter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lower, Some(self.upper))
        }
    }

    struct Collection {
        iter: Iter,
    }

    impl Collection {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let collection = Collection { iter: Iter { lower: 3, upper: 5 } };
    assert_eq!(collection.size_hint(), None);
}

#[test]
fn test_size_hint_none_upper() {
    struct Iter {
        size: usize,
    }

    impl Iter {
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, None)
        }
    }

    struct Collection {
        iter: Iter,
    }

    impl Collection {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let collection = Collection { iter: Iter { size: 4 } };
    assert_eq!(collection.size_hint(), None);
}

