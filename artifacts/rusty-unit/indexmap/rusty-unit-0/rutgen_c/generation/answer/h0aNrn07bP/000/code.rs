// Answer 0

#[test]
fn test_indexset_intersection_basic() {
    use std::collections::hash_map::RandomState;
    use std::hash::Hash;

    // Define necessary structures
    struct IndexSet<T, S = RandomState> {
        elements: Vec<T>,
        hasher: S,
    }

    impl<T: Eq + Hash, S: BuildHasher + Default> IndexSet<T, S> {
        fn new() -> Self {
            IndexSet {
                elements: Vec::new(),
                hasher: S::default(),
            }
        }

        fn insert(&mut self, value: T) {
            if !self.elements.contains(&value) {
                self.elements.push(value);
            }
        }

        fn intersection<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Vec<&'a T> 
        where
            S2: BuildHasher,
        {
            self.elements.iter().filter(|&&val| other.elements.contains(val)).collect()
        }
        
        fn cloned(self) -> IndexSet<T, S> {
            IndexSet {
                elements: self.elements.clone(),
                hasher: self.hasher,
            }
        }
    }

    impl<T: Eq + Hash + Clone, S1: BuildHasher + Default, S2: BuildHasher> 
    std::ops::BitAnd<&IndexSet<T, S2>> for &IndexSet<T, S1> {
        type Output = IndexSet<T, S1>;

        fn bitand(self, other: &IndexSet<T, S2>) -> Self::Output {
            let intersection: Vec<&T> = self.intersection(other);
            let mut result = IndexSet::new();
            for &item in intersection {
                result.insert(item.clone());
            }
            result
        }
    }

    // Create two IndexSet instances for testing
    let mut set1: IndexSet<i32> = IndexSet::new();
    let mut set2: IndexSet<i32> = IndexSet::new();

    // Add elements to the first set
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    // Add elements to the second set
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    // Perform intersection using the overloaded bitand operator
    let result = &set1 & &set2;

    // Assert that the result set contains the correct elements
    assert_eq!(result.elements, vec![2, 3]);
}

#[test]
fn test_indexset_intersection_empty() {
    use std::collections::hash_map::RandomState;
    use std::hash::Hash;

    struct IndexSet<T, S = RandomState> {
        elements: Vec<T>,
        hasher: S,
    }

    impl<T: Eq + Hash, S: BuildHasher + Default> IndexSet<T, S> {
        fn new() -> Self {
            IndexSet {
                elements: Vec::new(),
                hasher: S::default(),
            }
        }

        fn insert(&mut self, value: T) {
            if !self.elements.contains(&value) {
                self.elements.push(value);
            }
        }

        fn intersection<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Vec<&'a T> 
        where
            S2: BuildHasher,
        {
            self.elements.iter().filter(|&&val| other.elements.contains(val)).collect()
        }
        
        fn cloned(self) -> IndexSet<T, S> {
            IndexSet {
                elements: self.elements.clone(),
                hasher: self.hasher,
            }
        }
    }

    impl<T: Eq + Hash + Clone, S1: BuildHasher + Default, S2: BuildHasher> 
    std::ops::BitAnd<&IndexSet<T, S2>> for &IndexSet<T, S1> {
        type Output = IndexSet<T, S1>;

        fn bitand(self, other: &IndexSet<T, S2>) -> Self::Output {
            let intersection: Vec<&T> = self.intersection(other);
            let mut result = IndexSet::new();
            for &item in intersection {
                result.insert(item.clone());
            }
            result
        }
    }

    let set1: IndexSet<i32> = IndexSet::new(); // Empty set
    let mut set2: IndexSet<i32> = IndexSet::new();

    // Add elements to the second set
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    // Perform intersection using the overloaded bitand operator
    let result = &set1 & &set2;

    // Assert that the result set is empty
    assert_eq!(result.elements, Vec::<i32>::new());
}

