// Answer 0

#[test]
fn test_is_disjoint_with_no_common_elements() {
    use std::collections::hash_map::RandomState;

    struct SimpleIndexSet {
        elements: Vec<i32>,
    }

    impl MutableValues for SimpleIndexSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            unimplemented!()
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.elements.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            SimpleIndexSet {
                elements: Vec::new(),
            }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn contains(&self, value: &i32) -> bool {
            self.elements.contains(value)
        }

        fn push(&mut self, value: i32) {
            self.elements.push(value);
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.elements.iter()
        }
    }

    let set1 = SimpleIndexSet::new();
    let mut set2 = SimpleIndexSet::new();
    set2.push(6);
    set2.push(7);

    assert!(set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_with_common_elements() {
    use std::collections::hash_map::RandomState;

    struct SimpleIndexSet {
        elements: Vec<i32>,
    }

    impl MutableValues for SimpleIndexSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            unimplemented!()
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.elements.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            SimpleIndexSet {
                elements: Vec::new(),
            }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn contains(&self, value: &i32) -> bool {
            self.elements.contains(value)
        }

        fn push(&mut self, value: i32) {
            self.elements.push(value);
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.elements.iter()
        }
    }

    let mut set1 = SimpleIndexSet::new();
    set1.push(1);
    set1.push(2);

    let mut set2 = SimpleIndexSet::new();
    set2.push(2);
    set2.push(3);

    assert!(!set1.is_disjoint(&set2));
}

#[test]
fn test_is_disjoint_with_empty_sets() {
    use std::collections::hash_map::RandomState;

    struct SimpleIndexSet {
        elements: Vec<i32>,
    }

    impl MutableValues for SimpleIndexSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            unimplemented!()
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.elements.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    impl SimpleIndexSet {
        fn new() -> Self {
            SimpleIndexSet {
                elements: Vec::new(),
            }
        }

        fn len(&self) -> usize {
            self.elements.len()
        }

        fn contains(&self, value: &i32) -> bool {
            self.elements.contains(value)
        }

        fn iter(&self) -> impl Iterator<Item = &i32> {
            self.elements.iter()
        }
    }

    let set1 = SimpleIndexSet::new();
    let set2 = SimpleIndexSet::new();

    assert!(set1.is_disjoint(&set2));
}

