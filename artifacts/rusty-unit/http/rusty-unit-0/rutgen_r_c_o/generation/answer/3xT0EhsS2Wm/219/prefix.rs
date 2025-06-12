// Answer 0

#[test]
fn test_remove_extra_value_valid_case() {
    let mut raw_links_data: Vec<Option<Links>> = vec![Some(Links { next: 1, tail: 1 }), Some(Links { next: 2, tail: 2 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(1) }
    ];
    let raw_links = RawLinks(raw_links_data.as_mut_ptr());
    
    let idx = 1;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut raw_links_data: Vec<Option<Links>> = vec![Some(Links { next: 0, tail: 0 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) }
    ];
    let raw_links = RawLinks(raw_links_data.as_mut_ptr());
    
    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
#[should_panic]
fn test_remove_extra_value_invalid_index_case() {
    let mut raw_links_data: Vec<Option<Links>> = vec![Some(Links { next: 1, tail: 1 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) }
    ];
    let raw_links = RawLinks(raw_links_data.as_mut_ptr());
    
    let idx = 2; // This index is out of bounds.
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_with_panic_conditions() {
    let mut raw_links_data: Vec<Option<Links>> = vec![Some(Links { next: 0, tail: 0 }), Some(Links { next: 0, tail: 0 })];
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Extra(1), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Entry(0), next: Link::Extra(0) }
    ];
    let raw_links = RawLinks(raw_links_data.as_mut_ptr());
    
    let idx = 0;
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_with_no_extra_values() {
    let mut raw_links_data: Vec<Option<Links>> = Vec::new();
    let mut extra_values: Vec<ExtraValue<u32>> = Vec::new();
    let raw_links = RawLinks(raw_links_data.as_mut_ptr());
    
    let idx = 0; // No extra values to remove, should panic on bounds.
    let _ = remove_extra_value(raw_links, &mut extra_values, idx);
}

