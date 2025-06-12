// Answer 0

#[test]
fn test_values_mut_with_empty_header_map() {
    let mut map = HeaderMap::<String>::with_capacity(0);
    let values_mut = map.values_mut();
    assert_eq!(values_mut.inner.entry, 0);
}

#[test]
fn test_values_mut_with_single_entry() {
    let mut map = HeaderMap::<String>::with_capacity(1);
    map.insert("Key1".to_string(), "Value1".to_string());
    {
        let mut values = map.values_mut();
        assert_eq!(values.inner.map as *mut _, &mut map as *mut _);
        let value = values.inner.entry;
        assert_eq!(map.entries[value].value, "Value1");
        map.entries[value].value.push_str("-modified");
    }
    assert_eq!(map.get("Key1"), Some(&"Value1-modified".to_string()));
}

#[test]
fn test_values_mut_with_multiple_entries() {
    let mut map = HeaderMap::<String>::with_capacity(2);
    map.insert("Key1".to_string(), "Value1".to_string());
    map.insert("Key2".to_string(), "Value2".to_string());
    
    {
        let mut values = map.values_mut();
        for value in values.inner.iter_mut() {
            let entry_index = values.inner.entry;
            match entry_index {
                0 => map.entries[entry_index].value.push_str("-modified1"),
                1 => map.entries[entry_index].value.push_str("-modified2"),
                _ => (),
            }
        }
    }

    assert_eq!(map.get("Key1"), Some(&"Value1-modified1".to_string()));
    assert_eq!(map.get("Key2"), Some(&"Value2-modified2".to_string()));
} 

#[test]
fn test_values_mut_boundary_conditions() {
    let mut map = HeaderMap::<String>::with_capacity(2);
    map.insert("Key1".to_string(), "Value1".to_string());
    map.insert("Key2".to_string(), "Value2".to_string());
    
    {
        let mut values = map.values_mut();
        assert_eq!(values.inner.entry, 0);
        let value = values.inner.entry;
        let mut iter_counter = 0;

        while iter_counter < 2 {
            if let Some(value_mut) = values.inner.map.as_mut() {
                value_mut.value.push_str("-update");
                iter_counter += 1;
            }
        }
    }

    assert_eq!(map.get("Key1"), Some(&"Value1-update".to_string()));
    assert_eq!(map.get("Key2"), Some(&"Value2-update".to_string()));
}

