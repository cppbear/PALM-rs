// Answer 0

#[test]
fn test_iter_empty_set_slice() {
    struct SetSlice<T> {
        entries: Vec<T>,
    }

    struct Iter<'a, T> {
        data: &'a Vec<T>,
        index: usize,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(data: &'a Vec<T>) -> Self {
            Iter { data, index: 0 }
        }
        
        fn next(&mut self) -> Option<&'a T> {
            if self.index < self.data.len() {
                let result = &self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let set_slice: SetSlice<i32> = SetSlice { entries: vec![] };
    let mut iter = set_slice.iter();

    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_single_element_set_slice() {
    struct SetSlice<T> {
        entries: Vec<T>,
    }

    struct Iter<'a, T> {
        data: &'a Vec<T>,
        index: usize,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(data: &'a Vec<T>) -> Self {
            Iter { data, index: 0 }
        }
        
        fn next(&mut self) -> Option<&'a T> {
            if self.index < self.data.len() {
                let result = &self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let set_slice: SetSlice<i32> = SetSlice { entries: vec![42] };
    let mut iter = set_slice.iter();

    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_multiple_elements_set_slice() {
    struct SetSlice<T> {
        entries: Vec<T>,
    }

    struct Iter<'a, T> {
        data: &'a Vec<T>,
        index: usize,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(data: &'a Vec<T>) -> Self {
            Iter { data, index: 0 }
        }
        
        fn next(&mut self) -> Option<&'a T> {
            if self.index < self.data.len() {
                let result = &self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let set_slice: SetSlice<i32> = SetSlice { entries: vec![1, 2, 3] };
    let mut iter = set_slice.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

