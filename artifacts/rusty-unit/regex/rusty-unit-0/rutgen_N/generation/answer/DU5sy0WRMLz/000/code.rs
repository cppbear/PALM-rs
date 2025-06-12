// Answer 0

#[derive(Debug, PartialEq)]
struct FlagsItem {
    kind: String,
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
fn test_add_new_item() {
    let mut flags = Flags::new();
    let item = FlagsItem { kind: String::from("new_flag") };
    let result = flags.add_item(item);
    assert_eq!(result, None);
    assert_eq!(flags.items.len(), 1);
}

#[test]
fn test_add_duplicate_item() {
    let mut flags = Flags::new();
    let item = FlagsItem { kind: String::from("duplicate_flag") };
    let _ = flags.add_item(item.clone());
    let result = flags.add_item(item);
    assert_eq!(result, Some(0));
    assert_eq!(flags.items.len(), 1);
}

#[test]
fn test_add_two_different_items() {
    let mut flags = Flags::new();
    let item1 = FlagsItem { kind: String::from("flag_one") };
    let item2 = FlagsItem { kind: String::from("flag_two") };
    let result1 = flags.add_item(item1);
    let result2 = flags.add_item(item2);
    assert_eq!(result1, None);
    assert_eq!(result2, None);
    assert_eq!(flags.items.len(), 2);
}

