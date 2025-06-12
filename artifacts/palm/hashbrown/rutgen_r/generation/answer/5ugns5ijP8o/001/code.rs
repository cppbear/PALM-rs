// Answer 0

#[test]
fn test_into_iter_from_valid_input() {
    struct RawTable<T> {
        data: Vec<T>,
    }

    struct RawIter<T> {
        index: usize,
        data: Vec<T>,
    }

    struct RawIntoIter<T, A> {
        iter: RawIter<T>,
        allocation: Vec<T>,
        marker: std::marker::PhantomData<A>,
    }

    impl<T> RawTable<T> {
        fn into_allocation(self) -> Vec<T> {
            self.data
        }
    }

    impl<T> RawIter<T> {
        fn len(&self) -> usize {
            self.data.len() - self.index
        }
    }

    let table = RawTable { data: vec![1, 2, 3] };
    let iter = RawIter { index: 0, data: vec![1, 2, 3] };

    let result = unsafe { table.into_iter_from(iter) };
    
    assert_eq!(result.iter.len(), 3);
    assert_eq!(result.allocation.len(), 3);
    assert_eq!(result.iter.data, result.allocation);
}

#[test]
#[should_panic]
fn test_into_iter_from_panic_on_invalid_length() {
    struct RawTable<T> {
        data: Vec<T>,
    }

    struct RawIter<T> {
        index: usize,
        data: Vec<T>,
    }

    struct RawIntoIter<T, A> {
        iter: RawIter<T>,
        allocation: Vec<T>,
        marker: std::marker::PhantomData<A>,
    }

    impl<T> RawTable<T> {
        fn into_allocation(self) -> Vec<T> {
            self.data
        }
    }

    impl<T> RawIter<T> {
        fn len(&self) -> usize {
            self.data.len() - self.index
        }
    }

    let table = RawTable { data: vec![1, 2, 3] };
    let iter = RawIter { index: 0, data: vec![1, 2] }; // Invalid length

    unsafe { table.into_iter_from(iter) };
}

