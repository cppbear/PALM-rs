// Answer 0

#[test]
fn test_get_mut_existing_element() {
    struct Table<T> {
        elements: Vec<Option<T>>,
    }

    impl<T> Table<T> {
        fn new() -> Self {
            Table {
                elements: vec![None; 10],
            }
        }

        fn find<F>(&mut self, hash: u64, eq: F) -> Option<&mut Option<T>>
        where
            F: FnMut(&T) -> bool,
        {
            for element in &mut self.elements {
                if let Some(ref value) = element {
                    if eq(value) {
                        return Some(element);
                    }
                }
            }
            None
        }

        pub fn get_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { bucket.as_mut().unwrap().as_mut().unwrap() }),
                None => None,
            }
        }
    }

    let mut table = Table::new();
    table.elements[0] = Some(42); // Simulating an existing element

    let result = table.get_mut(0, |&x| x == 42);
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn test_get_mut_non_existing_element() {
    struct Table<T> {
        elements: Vec<Option<T>>,
    }

    impl<T> Table<T> {
        fn new() -> Self {
            Table {
                elements: vec![None; 10],
            }
        }

        fn find<F>(&mut self, hash: u64, eq: F) -> Option<&mut Option<T>>
        where
            F: FnMut(&T) -> bool,
        {
            for element in &mut self.elements {
                if let Some(ref value) = element {
                    if eq(value) {
                        return Some(element);
                    }
                }
            }
            None
        }

        pub fn get_mut(&mut self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { bucket.as_mut().unwrap().as_mut().unwrap() }),
                None => None,
            }
        }
    }

    let mut table = Table::new();
    table.elements[0] = Some(42); // Simulating an existing element

    let result = table.get_mut(0, |&x| x == 99);
    assert!(result.is_none());
}

