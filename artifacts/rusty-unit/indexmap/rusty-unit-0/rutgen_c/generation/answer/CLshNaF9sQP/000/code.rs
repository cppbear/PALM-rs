// Answer 0

#[test]
fn test_intersection() {
    use std::collections::hash_map::RandomState;
    use std::hash::Hash;

    struct IndexSet<T, S> {
        elements: Vec<T>,
        _marker: std::marker::PhantomData<S>,
    }

    impl<T: Hash + Eq, S: BuildHasher> IndexSet<T, S> {
        pub fn new() -> Self {
            IndexSet {
                elements: Vec::new(),
                _marker: std::marker::PhantomData,
            }
        }

        pub fn insert(&mut self, value: T) {
            self.elements.push(value);
        }

        pub fn intersection<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Intersection<'a, T, S2>
        where
            S2: BuildHasher,
        {
            Intersection::new(self, other)
        }
    }
    
    struct Intersection<'a, T, S> {
        iter: std::slice::Iter<'a, T>,
        other: &'a IndexSet<T, S>,
    }

    impl<'a, T, S> Intersection<'a, T, S> {
        pub fn new(iterable: &'a IndexSet<T, S>, other: &'a IndexSet<T, S>) -> Self {
            Intersection {
                iter: iterable.elements.iter(),
                other,
            }
        }
    }

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    let mut set_b: IndexSet<i32, RandomState> = IndexSet::new();
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);

    let intersection_result = set_a.intersection(&set_b);
    let mut result: Vec<_> = intersection_result.iter.collect();

    // We expect the intersection to contain 2 and 3 in order
    assert_eq!(result, vec![&2, &3]);
}

