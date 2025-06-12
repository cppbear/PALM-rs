// Answer 0

#[derive(Debug)]
struct ReBytes {
    data: Vec<String>,
}

impl ReBytes {
    fn new(data: Vec<String>) -> Self {
        ReBytes { data }
    }

    fn get(&self, i: usize) -> Option<&String> {
        self.data.get(i)
    }

    fn index(&self, i: usize) -> &[u8] {
        self.get(i).map(|m| m.as_bytes())
            .unwrap_or_else(|| panic!("no group at index '{}'", i))
    }
}

#[test]
fn test_index_valid() {
    let re_bytes = ReBytes::new(vec![String::from("test"), String::from("data")]);
    let result = re_bytes.index(0);
    assert_eq!(result, b"test");
}

#[test]
#[should_panic(expected = "no group at index '2'")]
fn test_index_out_of_bounds() {
    let re_bytes = ReBytes::new(vec![String::from("test")]);
    let _ = re_bytes.index(2);
}

