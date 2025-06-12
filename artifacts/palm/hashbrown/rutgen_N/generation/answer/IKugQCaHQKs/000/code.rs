// Answer 0

#[test]
fn test_get_mut_existing_element() {
    struct Element {
        value: u32,
    }

    struct Table {
        elements: Vec<Option<Element>>,
    }

    impl Table {
        fn new() -> Self {
            Table { elements: vec![None; 10] }
        }

        fn insert(&mut self, hash: u64, element: Element) {
            let index = (hash % self.elements.len() as u64) as usize;
            self.elements[index] = Some(element);
        }

        fn find(&self, hash: u64, eq: impl FnMut(&Element) -> bool) -> Option<&Element> {
            let index = (hash % self.elements.len() as u64) as usize;
            self.elements[index].as_ref().filter(|&elem| eq(elem))
        }

        fn get_mut(&mut self, hash: u64, eq: impl FnMut(&Element) -> bool) -> Option<&mut Element> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { &mut *(bucket as *const Element as *mut Element) }),
                None => None,
            }
        }
    }

    let mut table = Table::new();
    let element = Element { value: 42 };
    let hash = 5;

    table.insert(hash, element);

    let result = table.get_mut(hash, |e| e.value == 42);
    assert!(result.is_some());
    if let Some(elem) = result {
        elem.value = 100;
    }

    let updated_result = table.get_mut(hash, |e| e.value == 100);
    assert!(updated_result.is_some());
}

#[test]
fn test_get_mut_non_existing_element() {
    struct Element {
        value: u32,
    }

    struct Table {
        elements: Vec<Option<Element>>,
    }

    impl Table {
        fn new() -> Self {
            Table { elements: vec![None; 10] }
        }

        fn find(&self, hash: u64, eq: impl FnMut(&Element) -> bool) -> Option<&Element> {
            let index = (hash % self.elements.len() as u64) as usize;
            self.elements[index].as_ref().filter(|&elem| eq(elem))
        }

        fn get_mut(&mut self, hash: u64, eq: impl FnMut(&Element) -> bool) -> Option<&mut Element> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { &mut *(bucket as *const Element as *mut Element) }),
                None => None,
            }
        }
    }

    let mut table = Table::new();
    let hash = 5;

    let result = table.get_mut(hash, |e| e.value == 42);
    assert!(result.is_none());
}

