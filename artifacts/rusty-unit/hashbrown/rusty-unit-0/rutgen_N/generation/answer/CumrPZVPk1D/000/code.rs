// Answer 0

#[test]
fn test_iter_basic() {
    struct RawIter<T> {
        _marker: std::marker::PhantomData<T>,
    }

    struct MyContainer<T> {
        iter: RawIter<T>,
    }

    impl<T> MyContainer<T> {
        pub fn iter(&self) -> RawIter<T> {
            self.iter.clone()
        }
    }

    let container: MyContainer<i32> = MyContainer {
        iter: RawIter { _marker: std::marker::PhantomData },
    };

    let _iter_clone = container.iter();
}

#[test]
fn test_iter_clone() {
    struct RawIter<T> {
        data: Vec<T>,
        _marker: std::marker::PhantomData<T>,
    }

    #[derive(Clone)]
    struct MyContainer<T> {
        iter: RawIter<T>,
    }

    impl<T: Clone> MyContainer<T> {
        pub fn iter(&self) -> RawIter<T> {
            self.iter.clone()
        }
    }

    let container: MyContainer<i32> = MyContainer {
        iter: RawIter {
            data: vec![1, 2, 3],
            _marker: std::marker::PhantomData,
        },
    };

    let iter_clone = container.iter();
    assert_eq!(iter_clone.data.len(), 3);
}

