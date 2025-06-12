// Answer 0

#[derive(Debug, Clone, PartialEq)]
struct FlagsItem {
    kind: String,
}

struct Flags {
    items: Vec<FlagsItem>,
}

impl Flags {
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
fn test_add_item_duplicate() {
    let mut flags = Flags { items: vec![] };
    
    let item1 = FlagsItem { kind: "flag1".to_string() };
    let item2 = FlagsItem { kind: "flag1".to_string() }; // Duplicate item

    flags.add_item(item1);
    let result = flags.add_item(item2);

    assert_eq!(result, Some(0)); // Should return Some(0) as it's a duplicate
}

#[test]
fn test_add_item_no_duplicate() {
    let mut flags = Flags { items: vec![] };
    
    let item1 = FlagsItem { kind: "flag1".to_string() };
    let item2 = FlagsItem { kind: "flag2".to_string() }; // Unique item

    flags.add_item(item1);
    let result = flags.add_item(item2);

    assert_eq!(result, None); // Should return None as it's not a duplicate
}

#[test]
fn test_add_item_multiple_duplicates() {
    let mut flags = Flags { items: vec![] };
    
    let item1 = FlagsItem { kind: "flag1".to_string() };
    let item2 = FlagsItem { kind: "flag2".to_string() };
    let item3 = FlagsItem { kind: "flag1".to_string() }; // Duplicate of item1

    flags.add_item(item1);
    flags.add_item(item2);
    let result = flags.add_item(item3);

    assert_eq!(result, Some(0)); // Should return Some(0) as it's a duplicate of item1
}

