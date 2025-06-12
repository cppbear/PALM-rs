// Answer 0

#[derive(Debug)]
struct MockSeqAccess {
    items: Vec<i32>,
    current_index: usize,
}

impl MockSeqAccess {
    fn new(items: Vec<i32>) -> Self {
        MockSeqAccess {
            items,
            current_index: 0,
        }
    }
}

impl<'de> serde::de::SeqAccess<'de> for MockSeqAccess {
    type Error = serde::de::value::Error;

    fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        if self.current_index < self.items.len() {
            let value = self.items[self.current_index];
            self.current_index += 1;
            Ok(Some(value as T))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_next_element_some() {
    let mut seq_access = MockSeqAccess::new(vec![1, 2, 3]);
    
    let element: Option<i32> = seq_access.next_element().unwrap();
    assert_eq!(element, Some(1));

    let element: Option<i32> = seq_access.next_element().unwrap();
    assert_eq!(element, Some(2));
}

#[test]
fn test_next_element_none() {
    let mut seq_access = MockSeqAccess::new(vec![1, 2, 3]);
    
    let _ = seq_access.next_element().unwrap();
    let _ = seq_access.next_element().unwrap();
    let _ = seq_access.next_element().unwrap();
    
    let element: Option<i32> = seq_access.next_element().unwrap();
    assert_eq!(element, None);
}

