// Answer 0

#[test]
fn test_iter() {
    struct RawIter<T> {
        _marker: std::marker::PhantomData<T>,
    }

    struct HashBrown<T> {
        iter: RawIter<T>,
    }

    impl<T> HashBrown<T> {
        pub fn new(iter: RawIter<T>) -> Self {
            HashBrown { iter }
        }

        pub fn iter(&self) -> RawIter<T> {
            self.iter.clone()
        }
    }

    impl<T> Clone for RawIter<T> {
        fn clone(&self) -> Self {
            RawIter { _marker: std::marker::PhantomData }
        }
    }

    let raw_iter = RawIter { _marker: std::marker::PhantomData };
    let hashbrown = HashBrown::new(raw_iter);
    let cloned_iter = hashbrown.iter();

    // Since RawIter does not hold any state, we can assess that the cloned iteration works.
    assert_eq!(std::mem::size_of_val(&hashbrown.iter), std::mem::size_of_val(&cloned_iter));
}

