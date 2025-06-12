// Answer 0

#[derive(Debug)]
struct ClassSetItem {
    items: Vec<u32>, // Assuming the items are of type u32 for the purpose of this test
    span: usize, // Example field
}

impl ClassSetItem {
    pub fn into_item(mut self) -> ClassSetItem {
        match self.items.len() {
            0 => ClassSetItem { items: vec![], span: self.span },
            1 => {
                let item = self.items.pop().unwrap();
                ClassSetItem { items: vec![item], span: self.span }
            },
            _ => ClassSetItem { items: self.items, span: self.span },
        }
    }
}

#[test]
fn test_into_item_empty() {
    let item = ClassSetItem { items: vec![], span: 0 };
    let result = item.into_item();
    assert_eq!(result.items.len(), 0);
    assert_eq!(result.span, 0);
}

#[test]
fn test_into_item_single() {
    let item = ClassSetItem { items: vec![42], span: 1 };
    let result = item.into_item();
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0], 42);
    assert_eq!(result.span, 1);
}

#[test]
fn test_into_item_multiple() {
    let item = ClassSetItem { items: vec![1, 2, 3], span: 2 };
    let result = item.into_item();
    assert_eq!(result.items.len(), 3);
    assert_eq!(result.items, vec![1, 2, 3]);
    assert_eq!(result.span, 2);
}

