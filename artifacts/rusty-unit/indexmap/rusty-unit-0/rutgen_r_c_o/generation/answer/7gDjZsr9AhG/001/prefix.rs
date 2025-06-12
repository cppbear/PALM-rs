// Answer 0

#[test]
fn test_swap_remove_full_existing_value() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: vec![] }
        }

        fn swap_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            if let Some(pos) = self.elements.iter().position(|x| x == value) {
                let last = self.elements.pop().unwrap();
                if pos != self.elements.len() {
                    self.elements[pos] = last;
                }
                Some((pos, value.clone()))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    set.elements.extend(1..=1000);
    let value_to_remove = 500;

    let result = set.swap_remove_full(&value_to_remove);
}

#[test]
fn test_swap_remove_full_no_existing_value() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: vec![] }
        }

        fn swap_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            if let Some(pos) = self.elements.iter().position(|x| x == value) {
                let last = self.elements.pop().unwrap();
                if pos != self.elements.len() {
                    self.elements[pos] = last;
                }
                Some((pos, value.clone()))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    set.elements.extend(1..=1000);
    let value_to_remove = 1001;

    let result = set.swap_remove_full(&value_to_remove);
}

#[test]
fn test_swap_remove_full_remove_first_element() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: vec![] }
        }

        fn swap_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            if let Some(pos) = self.elements.iter().position(|x| x == value) {
                let last = self.elements.pop().unwrap();
                if pos != self.elements.len() {
                    self.elements[pos] = last;
                }
                Some((pos, value.clone()))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    for i in 1..=1000 {
        set.elements.push(i);
    }
    let value_to_remove = 1;

    let result = set.swap_remove_full(&value_to_remove);
} 

#[test]
fn test_swap_remove_full_remove_last_element() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: vec![] }
        }

        fn swap_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            if let Some(pos) = self.elements.iter().position(|x| x == value) {
                let last = self.elements.pop().unwrap();
                if pos != self.elements.len() {
                    self.elements[pos] = last;
                }
                Some((pos, value.clone()))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    for i in 1..=1000 {
        set.elements.push(i);
    }
    let value_to_remove = 1000;

    let result = set.swap_remove_full(&value_to_remove);
} 

#[test]
fn test_swap_remove_full_empty_set() {
    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { elements: vec![] }
        }

        fn swap_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            if let Some(pos) = self.elements.iter().position(|x| x == value) {
                let last = self.elements.pop().unwrap();
                if pos != self.elements.len() {
                    self.elements[pos] = last;
                }
                Some((pos, value.clone()))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    let value_to_remove = 1;

    let result = set.swap_remove_full(&value_to_remove);
}

