// Answer 0

#[test]
fn test_remove_extra_value_with_valid_conditions() {
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 1024])));
    let mut extra_values: Vec<ExtraValue<u32>> = Vec::with_capacity(1024);
    
    for i in 0..1024 {
        extra_values.push(ExtraValue {
            value: i as u32,
            prev: Link::Extra((i + 1) % 1024),
            next: Link::Entry(i % 512),
        });
    }

    let idx = 150; // 100 < idx < 500
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_with_sequential_indices() {
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 1024])));
    let mut extra_values: Vec<ExtraValue<u32>> = Vec::with_capacity(1024);
    
    for i in 0..1024 {
        extra_values.push(ExtraValue {
            value: i as u32,
            prev: Link::Extra((i + 1) % 1024),
            next: Link::Entry(i % 512),
        });
    }

    let idx = 250; // 100 < idx < 500
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_with_upper_boundary_index() {
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 1024])));
    let mut extra_values: Vec<ExtraValue<u32>> = Vec::with_capacity(1024);
    
    for i in 0..1024 {
        extra_values.push(ExtraValue {
            value: i as u32,
            prev: Link::Extra((i + 1) % 1024),
            next: Link::Entry(i % 512),
        });
    }

    let idx = 499; // 100 < idx < 500
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_with_empty_raw_links() {
    let mut raw_links: RawLinks<u32> = RawLinks(Box::into_raw(Box::new([None; 1024])));
    let mut extra_values: Vec<ExtraValue<u32>> = Vec::with_capacity(1024);
    
    for i in 0..1024 {
        extra_values.push(ExtraValue {
            value: i as u32,
            prev: Link::Entry(i % 512),
            next: Link::Extra((i + 1) % 1024),
        });
    }

    let idx = 200; // 100 < idx < 500
    let result = remove_extra_value(raw_links, &mut extra_values, idx);
}

