// Answer 0

#[derive(Debug, Clone, PartialEq)]
struct FlagsItem {
    kind: u32,
}

struct Flags {
    items: Vec<FlagsItem>,
}

impl Flags {
    pub fn new() -> Self {
        Flags { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: FlagsItem) -> Option<usize> {
        for (i, x) in self.items.iter().enumerate() {
            if x.kind == item.kind {
                return Some(i);
            }
        }
        self.items.push(item);
        None
    }
}

#[test]
fn test_add_item_empty() {
    let mut flags = Flags::new();
    let item = FlagsItem { kind: 1 };
    let result = flags.add_item(item);
    assert!(result.is_none());
}

#[test]
fn test_add_item_no_duplicates() {
    let mut flags = Flags::new();
    let item1 = FlagsItem { kind: 1 };
    let item2 = FlagsItem { kind: 2 };
    let result1 = flags.add_item(item1);
    let result2 = flags.add_item(item2);
    
    assert!(result1.is_none());
    assert!(result2.is_none());
}

#[test]
fn test_add_item_with_duplicate() {
    let mut flags = Flags::new();
    let item = FlagsItem { kind: 1 };
    
    // First addition
    let result1 = flags.add_item(item.clone());
    assert!(result1.is_none());
    
    // Attempt to add duplicate
    let result2 = flags.add_item(item);
    assert_eq!(result2, Some(0)); // Should return index 0
}

