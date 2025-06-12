// Answer 0

#[derive(Debug)]
struct Core {
    capacity: usize,
}

impl Core {
    fn shrink_to(&mut self, new_capacity: usize) {
        if new_capacity < self.capacity {
            self.capacity = new_capacity;
        }
    }
}

struct MyMap {
    core: Core,
}

impl MyMap {
    fn new(capacity: usize) -> Self {
        MyMap {
            core: Core { capacity },
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.core.shrink_to(0);
    }

    fn capacity(&self) -> usize {
        self.core.capacity
    }
}

#[test]
fn test_shrink_to_fit() {
    let mut my_map = MyMap::new(10);
    assert_eq!(my_map.capacity(), 10);
    my_map.shrink_to_fit();
    assert_eq!(my_map.capacity(), 0);
}

#[test]
fn test_shrink_to_fit_already_min_capacity() {
    let mut my_map = MyMap::new(0);
    assert_eq!(my_map.capacity(), 0);
    my_map.shrink_to_fit();
    assert_eq!(my_map.capacity(), 0);
}

