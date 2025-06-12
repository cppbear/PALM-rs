// Answer 0

#[derive(Debug, PartialEq, Eq, Clone)]
struct IndexSet<T, S> {
    items: Vec<T>,
    // Other fields and associated methods as needed
}

impl<T: PartialEq, S> IndexSet<T, S> {
    fn intersection<'a>(&'a self, other: &'a IndexSet<T, S>) -> Vec<&'a T> {
        self.items.iter().filter(|item| other.items.contains(*item)).collect()
    }

    fn bitand(self, other: &IndexSet<T, S>) -> IndexSet<T, S> {
        let items = self.intersection(other).cloned().collect();
        IndexSet { items }
    }
}

#[test]
fn test_bitand() {
    let set_a = IndexSet {
        items: vec![1, 2, 3, 4],
    };
    let set_b = IndexSet {
        items: vec![3, 4, 5, 6],
    };

    let result = set_a.bitand(&set_b);
    let expected = IndexSet {
        items: vec![3, 4],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_bitand_empty() {
    let set_a = IndexSet {
        items: vec![],
    };
    let set_b = IndexSet {
        items: vec![3, 4, 5, 6],
    };

    let result = set_a.bitand(&set_b);
    let expected = IndexSet {
        items: vec![],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_bitand_no_common_elements() {
    let set_a = IndexSet {
        items: vec![1, 2],
    };
    let set_b = IndexSet {
        items: vec![3, 4],
    };

    let result = set_a.bitand(&set_b);
    let expected = IndexSet {
        items: vec![],
    };
    assert_eq!(result, expected);
}

