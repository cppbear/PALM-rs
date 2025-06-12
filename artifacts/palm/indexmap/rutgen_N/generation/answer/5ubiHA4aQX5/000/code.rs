// Answer 0

#[test]
fn test_new_with_non_empty_iter() {
    use std::vec;

    struct Bucket<T> {
        items: Vec<T>,
    }

    struct Iter<T> {
        iter: vec::Drain<'static, Bucket<T>>,
    }

    impl<T> Iter<T> {
        pub(super) fn new(iter: vec::Drain<'static, Bucket<T>>) -> Self {
            Self { iter }
        }
    }

    let mut bucket = Bucket { items: vec![1, 2, 3] };
    let drain = bucket.items.drain(..);
    let iter = Iter::new(drain);

    assert!(iter.iter.as_mut().next().is_some());
}

#[test]
fn test_new_with_empty_iter() {
    use std::vec;

    struct Bucket<T> {
        items: Vec<T>,
    }

    struct Iter<T> {
        iter: vec::Drain<'static, Bucket<T>>,
    }

    impl<T> Iter<T> {
        pub(super) fn new(iter: vec::Drain<'static, Bucket<T>>) -> Self {
            Self { iter }
        }
    }

    let mut bucket = Bucket { items: Vec::new() };
    let drain = bucket.items.drain(..);
    let iter = Iter::new(drain);
    
    assert!(iter.iter.as_mut().next().is_none());
}

