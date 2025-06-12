// Answer 0

#[derive(Debug)]
struct MySet {
    map: Vec<i32>,
}

impl MySet {
    fn new() -> Self {
        MySet { map: Vec::new() }
    }

    fn from_vec(vec: Vec<i32>) -> Self {
        MySet { map: vec }
    }

    fn reverse(&mut self) {
        self.map.reverse()
    }
}

#[test]
fn test_reverse_empty_set() {
    let mut set = MySet::new();
    set.reverse();
    assert_eq!(set.map, Vec::<i32>::new());
}

#[test]
fn test_reverse_single_element_set() {
    let mut set = MySet::from_vec(vec![1]);
    set.reverse();
    assert_eq!(set.map, vec![1]);
}

#[test]
fn test_reverse_multiple_elements_set() {
    let mut set = MySet::from_vec(vec![1, 2, 3, 4]);
    set.reverse();
    assert_eq!(set.map, vec![4, 3, 2, 1]);
}

#[test]
fn test_reverse_identical_elements_set() {
    let mut set = MySet::from_vec(vec![2, 2, 2, 2]);
    set.reverse();
    assert_eq!(set.map, vec![2, 2, 2, 2]);
}

