// Answer 0


#[derive(Clone)]
struct MySet {
    items: Vec<i32>,
}

impl MySet {
    fn new() -> Self {
        MySet { items: Vec::new() }
    }
    
    fn insert(&mut self, value: i32) {
        self.items.push(value);
    }
}

impl std::fmt::Debug for MySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone().items).finish()
    }
}

#[test]
fn test_empty_set() {
    let set = MySet::new();
    let result = format!("{:?}", set);
    assert_eq!(result, "[]");
}

#[test]
fn test_single_element_set() {
    let mut set = MySet::new();
    set.insert(42);
    let result = format!("{:?}", set);
    assert_eq!(result, "[42]");
}

#[test]
fn test_multiple_elements_set() {
    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let result = format!("{:?}", set);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_clone_set() {
    let mut set = MySet::new();
    set.insert(100);
    let cloned_set = set.clone();
    let result = format!("{:?}", cloned_set);
    assert_eq!(result, "[100]");
}

#[should_panic]
fn test_panic_on_empty_format() {
    let set = MySet::new();
    let _ = format!("{:?}", set);
}


