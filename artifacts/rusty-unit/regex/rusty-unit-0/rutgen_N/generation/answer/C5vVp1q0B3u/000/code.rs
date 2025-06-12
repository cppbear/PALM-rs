// Answer 0

#[derive(Debug)]
struct Inner(Arc<HashMap<String, usize>>);

impl Inner {
    pub fn new() -> Self {
        let map = Arc::new(HashMap::new());
        Inner(map)
    }

    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
        &self.0
    }
}

#[derive(Debug)]
struct Outer(Inner);

impl Outer {
    pub fn new() -> Self {
        Outer(Inner::new())
    }

    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
        self.0.capture_name_idx()
    }
}

#[test]
fn test_capture_name_idx_empty() {
    let outer = Outer::new();
    let idx = outer.capture_name_idx();
    assert_eq!(idx.len(), 0);
}

#[test]
fn test_capture_name_idx_with_data() {
    let mut map = HashMap::new();
    map.insert("test".to_string(), 1);
    let arc_map = Arc::new(map);
    let inner = Inner(arc_map);
    let outer = Outer(inner);
    let idx = outer.capture_name_idx();
    assert_eq!(idx.len(), 1);
    assert_eq!(idx.get("test"), Some(&1));
}

