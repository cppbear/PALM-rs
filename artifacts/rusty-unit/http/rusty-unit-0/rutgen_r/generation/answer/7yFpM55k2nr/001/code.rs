// Answer 0

#[derive(Default)]
struct HeaderMap {
    entries: Vec<(String, String)>,
    extra_values: Vec<String>,
    danger: Danger,
    indices: Vec<Pos>,
}

#[derive(Default)]
struct Pos {
    // Assume Pos structure has necessary fields and methods
}

impl Pos {
    pub fn none() -> Self {
        Pos {}
    }
}

#[derive(PartialEq)]
enum Danger {
    Green,
    Yellow,
    Red,
}

impl HeaderMap {
    pub fn new() -> Self {
        HeaderMap {
            entries: Vec::new(),
            extra_values: Vec::new(),
            danger: Danger::Green,
            indices: vec![Pos::none(); 10], // Initialized with 10 positions
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.extra_values.clear();
        self.danger = Danger::Green;

        for e in self.indices.iter_mut() {
            *e = Pos::none();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.extra_values.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }
}

#[test]
fn test_clear_on_empty_header_map() {
    let mut map = HeaderMap::new();
    map.clear();
    
    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_on_non_empty_header_map() {
    let mut map = HeaderMap::new();
    map.entries.push(("key1".to_string(), "value1".to_string()));
    map.extra_values.push("extraValue".to_string());

    map.clear();

    assert!(map.is_empty());
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_reinitialize_indices() {
    let mut map = HeaderMap::new();
    map.indices = vec![Pos {}, Pos {}, Pos {}]; // Set to non-default state

    map.clear();

    assert!(map.indices.iter().all(|e| *e == Pos::none()));
}

