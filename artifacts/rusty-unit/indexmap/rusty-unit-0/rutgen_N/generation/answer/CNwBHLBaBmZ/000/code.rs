// Answer 0

#[test]
fn test_sort_by_stability() {
    use std::collections::BTreeMap;
    use std::cmp::Ordering;

    struct MySet {
        map: Vec<(i32, i32)>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { map: Vec::new() }
        }

        fn sort_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&(i32, i32), &(i32, i32)) -> Ordering,
        {
            self.map.sort_by(move |a, b| cmp(a, b));
        }
    }

    let mut set = MySet::new();
    set.map.push((1, 10));
    set.map.push((2, 20));
    set.map.push((3, 15));
    set.map.push((2, 25));

    set.sort_by(|a, b| {
        if a.0 == b.0 {
            Ordering::Equal
        } else {
            a.0.cmp(&b.0)
        }
    });

    assert_eq!(set.map, vec![(1, 10), (2, 20), (2, 25), (3, 15)]);
}

#[test]
fn test_sort_empty_set() {
    struct MySet {
        map: Vec<(i32, i32)>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { map: Vec::new() }
        }

        fn sort_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&(i32, i32), &(i32, i32)) -> Ordering,
        {
            self.map.sort_by(move |a, b| cmp(a, b));
        }
    }

    let mut set = MySet::new();
    set.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(set.map.is_empty());
}

#[test]
fn test_sort_by_reverse_order() {
    struct MySet {
        map: Vec<(i32, i32)>,
    }

    impl MySet {
        fn new() -> Self {
            MySet { map: Vec::new() }
        }

        fn sort_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&(i32, i32), &(i32, i32)) -> Ordering,
        {
            self.map.sort_by(move |a, b| cmp(a, b));
        }
    }

    let mut set = MySet::new();
    set.map.push((3, 30));
    set.map.push((1, 10));
    set.map.push((2, 20));

    set.sort_by(|a, b| b.0.cmp(&a.0));

    assert_eq!(set.map, vec![(3, 30), (2, 20), (1, 10)]);
}

