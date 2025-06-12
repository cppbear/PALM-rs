// Answer 0

#[test]
fn test_add_item_new_item() {
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

    let mut flags = Flags::new();
    let item1 = FlagsItem { kind: 1 };
    let item2 = FlagsItem { kind: 2 };
    
    // Add a new item, expect None as it's not a duplicate
    let result1 = flags.add_item(item1);
    assert_eq!(result1, None);
    
    // Add another new item, expect None as it's also not a duplicate
    let result2 = flags.add_item(item2);
    assert_eq!(result2, None);
}

#[test]
fn test_add_item_duplicate() {
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

    let mut flags = Flags::new();
    let item1 = FlagsItem { kind: 1 };
    
    // Add a new item, expect None as it's not a duplicate
    flags.add_item(item1);
    
    // Try to add the same item again, expect Some(0) as it's a duplicate now
    let duplicate_item = FlagsItem { kind: 1 };
    let result = flags.add_item(duplicate_item);
    assert_eq!(result, Some(0));
}

