// Answer 0

#[derive(Debug)]
struct ExtraValue<T> {
    value: T,
}

struct HeaderMap<T> {
    extra_values: Vec<ExtraValue<T>>,
}

impl<T> HeaderMap<T> {
    fn raw_links(&self) -> &Vec<ExtraValue<T>> {
        &self.extra_values
    }

    fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<T> {
        let raw_links = self.raw_links();
        remove_extra_value(raw_links, &mut self.extra_values, idx)
    }
}

fn remove_extra_value<T>(raw_links: &Vec<ExtraValue<T>>, extra_values: &mut Vec<ExtraValue<T>>, idx: usize) -> ExtraValue<T> {
    if idx < extra_values.len() {
        extra_values.remove(idx)
    } else {
        panic!("Index out of bounds");
    }
}

#[test]
fn test_remove_extra_value_valid_index() {
    let mut header_map = HeaderMap {
        extra_values: vec![
            ExtraValue { value: 1 },
            ExtraValue { value: 2 },
            ExtraValue { value: 3 },
        ],
    };

    let removed = header_map.remove_extra_value(1);
    assert_eq!(removed.value, 2);
    assert_eq!(header_map.extra_values.len(), 2);
    assert_eq!(header_map.extra_values[0].value, 1);
    assert_eq!(header_map.extra_values[1].value, 3);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_remove_extra_value_invalid_index() {
    let mut header_map = HeaderMap {
        extra_values: vec![
            ExtraValue { value: 1 },
            ExtraValue { value: 2 },
        ],
    };

    // Attempt to remove an index that is out of bounds
    header_map.remove_extra_value(5);
}

