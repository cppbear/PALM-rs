// Answer 0

#[test]
fn test_insert_occupied_mult() {
    struct TestEntry<T> {
        value: T,
        links: Option<Links<T>>,
    }

    struct Links<T> {
        next: usize,
    }

    struct TestMap<T> {
        entries: Vec<TestEntry<T>>,
        extra_values: Vec<T>,
    }

    impl<T> TestMap<T> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn raw_links(&self) -> &[T] {
            &self.extra_values
        }

        fn insert_occupied_mult(&mut self, index: usize, value: T) -> ValueDrain<'_, T> {
            use std::mem;

            let old;
            let links;

            {
                let entry = &mut self.entries[index];

                old = mem::replace(&mut entry.value, value);
                links = entry.links.take();
            }

            let raw_links = self.raw_links();
            let extra_values = &mut self.extra_values;

            let next = links.map(|l| drain_all_extra_values(raw_links, extra_values, l.next).into_iter());

            ValueDrain {
                first: Some(old),
                next,
                lt: PhantomData,
            }
        }
    }

    struct ValueDrain<'a, T> {
        first: Option<T>,
        next: Option<Box<dyn Iterator<Item = T> + 'a>>,
        lt: std::marker::PhantomData<&'a T>,
    }

    fn drain_all_extra_values<T>(raw_links: &[T], extra_values: &mut Vec<T>, index: usize) -> Vec<T> {
        // Placeholder for function logic
        extra_values.clone()
    }

    let mut map = TestMap::<i32>::new();
    map.entries.push(TestEntry { value: 1, links: Some(Links { next: 0 }) });
    map.entries.push(TestEntry { value: 2, links: None });

    let drain = map.insert_occupied_mult(0, 3);
    
    assert_eq!(drain.first, Some(1));
    assert_eq!(map.entries[0].value, 3);
    assert!(drain.next.is_some());
}

#[test]
fn test_insert_occupied_mult_empty_links() {
    struct TestEntry<T> {
        value: T,
        links: Option<Links<T>>,
    }

    struct Links<T> {
        next: usize,
    }

    struct TestMap<T> {
        entries: Vec<TestEntry<T>>,
        extra_values: Vec<T>,
    }

    impl<T> TestMap<T> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn raw_links(&self) -> &[T] {
            &self.extra_values
        }

        fn insert_occupied_mult(&mut self, index: usize, value: T) -> ValueDrain<'_, T> {
            use std::mem;

            let old;
            let links;

            {
                let entry = &mut self.entries[index];

                old = mem::replace(&mut entry.value, value);
                links = entry.links.take();
            }

            let raw_links = self.raw_links();
            let extra_values = &mut self.extra_values;

            let next = links.map(|l| drain_all_extra_values(raw_links, extra_values, l.next).into_iter());

            ValueDrain {
                first: Some(old),
                next,
                lt: PhantomData,
            }
        }
    }

    struct ValueDrain<'a, T> {
        first: Option<T>,
        next: Option<Box<dyn Iterator<Item = T> + 'a>>,
        lt: std::marker::PhantomData<&'a T>,
    }

    fn drain_all_extra_values<T>(raw_links: &[T], extra_values: &mut Vec<T>, index: usize) -> Vec<T> {
        // Placeholder for function logic
        extra_values.clone()
    }

    let mut map = TestMap::<i32>::new();
    map.entries.push(TestEntry { value: 1, links: None });

    let drain = map.insert_occupied_mult(0, 3);

    assert_eq!(drain.first, Some(1));
    assert_eq!(map.entries[0].value, 3);
    assert!(drain.next.is_none());
}

