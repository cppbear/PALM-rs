// Answer 0

#[derive(Default)]
struct Indices {
    data: Vec<usize>,
}

struct MyCollection {
    indices: Indices,
}

impl MyCollection {
    pub(crate) fn len(&self) -> usize {
        self.indices.data.len()
    }
}

#[test]
fn test_len_empty() {
    let collection = MyCollection {
        indices: Indices::default(),
    };
    assert_eq!(collection.len(), 0);
}

#[test]
fn test_len_single_element() {
    let collection = MyCollection {
        indices: Indices { data: vec![1] },
    };
    assert_eq!(collection.len(), 1);
}

#[test]
fn test_len_multiple_elements() {
    let collection = MyCollection {
        indices: Indices { data: vec![1, 2, 3, 4, 5] },
    };
    assert_eq!(collection.len(), 5);
}

#[test]
fn test_len_large_collection() {
    let collection = MyCollection {
        indices: Indices { data: (0..1000).collect() },
    };
    assert_eq!(collection.len(), 1000);
}

