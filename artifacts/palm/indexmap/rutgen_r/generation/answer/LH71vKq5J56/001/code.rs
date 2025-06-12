// Answer 0

#[derive(Debug)]
struct Map {
    indices: Vec<usize>,
}

impl Map {
    fn new() -> Self {
        Map {
            indices: Vec::new(),
        }
    }

    pub fn index(&self) -> usize {
        self.indices.len()
    }
}

#[test]
fn test_index_empty() {
    let map = Map::new();
    assert_eq!(map.index(), 0);
}

#[test]
fn test_index_single_element() {
    let mut map = Map::new();
    map.indices.push(0);
    assert_eq!(map.index(), 1);
}

#[test]
fn test_index_multiple_elements() {
    let mut map = Map::new();
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    assert_eq!(map.index(), 3);
}

#[test]
fn test_index_large_number_of_elements() {
    let mut map = Map::new();
    for i in 0..1000 {
        map.indices.push(i);
    }
    assert_eq!(map.index(), 1000);
}

#[test]
fn test_index_after_clearing() {
    let mut map = Map::new();
    map.indices.push(1);
    map.indices.clear();
    assert_eq!(map.index(), 0);
}

