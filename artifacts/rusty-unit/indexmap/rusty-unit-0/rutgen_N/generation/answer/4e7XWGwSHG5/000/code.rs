// Answer 0

#[test]
fn test_sort_unstable_by() {
    use std::collections::HashMap;
    use std::cmp::Ordering;

    struct MySet {
        map: Vec<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet { map: Vec::new() }
        }

        pub fn insert(&mut self, value: i32) {
            self.map.push(value);
        }

        pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            self.map.sort_unstable_by(move |a, b| cmp(a, b));
        }
    }

    let mut set = MySet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    // Sorting in ascending order
    set.sort_unstable_by(|a, b| a.cmp(b));
    assert_eq!(set.map, vec![1, 2, 3]);

    // Sorting in descending order
    set.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(set.map, vec![3, 2, 1]);
}

#[test]
fn test_sort_unstable_by_empty() {
    use std::cmp::Ordering;

    struct MySet {
        map: Vec<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet { map: Vec::new() }
        }

        pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            self.map.sort_unstable_by(move |a, b| cmp(a, b));
        }
    }

    let mut set = MySet::new();
    set.sort_unstable_by(|a, b| a.cmp(b));
    assert!(set.map.is_empty());
}

