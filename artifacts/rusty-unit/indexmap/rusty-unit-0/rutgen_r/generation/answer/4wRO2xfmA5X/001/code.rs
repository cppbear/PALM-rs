// Answer 0


#[derive(Debug)]
struct TestMap {
    core: Vec<(usize, usize)>,
}

impl TestMap {
    pub fn new() -> Self {
        TestMap { core: Vec::new() }
    }
    
    pub fn push(&mut self, key: usize, value: usize) {
        self.core.push((key, value));
    }
    
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        if a >= self.core.len() || b >= self.core.len() {
            panic!("Index out of bounds");
        }
        self.core.swap(a, b);
    }
}

#[test]
fn test_swap_indices_valid() {
    let mut map = TestMap::new();
    map.push(1, 10);
    map.push(2, 20);
    map.push(3, 30);
    
    map.swap_indices(0, 1);
    assert_eq!(map.core, vec![(2, 20), (1, 10), (3, 30)]);
    
    map.swap_indices(1, 2);
    assert_eq!(map.core, vec![(2, 20), (3, 30), (1, 10)]);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_swap_indices_index_out_of_bounds_a() {
    let mut map = TestMap::new();
    map.push(1, 10);
    map.push(2, 20);
    
    map.swap_indices(0, 2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_swap_indices_index_out_of_bounds_b() {
    let mut map = TestMap::new();
    map.push(1, 10);
    map.push(2, 20);
    
    map.swap_indices(2, 1);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_swap_indices_both_out_of_bounds() {
    let mut map = TestMap::new();
    map.push(1, 10);
    
    map.swap_indices(1, 2);
}


