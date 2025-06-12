// Answer 0

#[derive(Default)]
struct Set {
    map: std::collections::HashMap<u32, ()>,
}

impl Set {
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.map.shrink_to(min_capacity);
    }
}

#[test]
fn test_shrink_to_below_current_capacity() {
    let mut set = Set::default();
    set.map.insert(1, ());
    set.map.insert(2, ());
    let initial_capacity = set.map.capacity();

    set.shrink_to(initial_capacity - 1);
    
    assert!(set.map.capacity() >= initial_capacity - 1);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut set = Set::default();
    set.map.insert(1, ());
    set.map.insert(2, ());
    let initial_capacity = set.map.capacity();

    set.shrink_to(initial_capacity);
    
    assert_eq!(set.map.capacity(), initial_capacity);
}

#[test]
fn test_shrink_to_zero_capacity() {
    let mut set = Set::default();
    set.map.insert(1, ());
    set.map.insert(2, ());
    
    set.shrink_to(0);
    
    assert!(set.map.capacity() >= 0);
}

#[test]
fn test_shrink_to_greater_than_current_capacity() {
    let mut set = Set::default();
    set.map.insert(1, ());
    
    let initial_capacity = set.map.capacity();
    
    set.shrink_to(initial_capacity + 1);
    
    assert!(set.map.capacity() >= initial_capacity + 1);
}

