// Answer 0

#[test]
fn test_remove_extra_value_basic_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket::<u32>::default(); 128])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(0) },
        ExtraValue { value: 2, prev: Link::Extra(0), next: Link::Entry(1) },
    ];

    unsafe {
        raw_links.0.as_mut().unwrap()[1] = Some(Links { next: 2, tail: 2 });
    }
    
    remove_extra_value(raw_links, &mut extra_values, 0);
}

#[test]
fn test_remove_extra_value_updated_links_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket::<u32>::default(); 128])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(3), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(2) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Extra(3) },
        ExtraValue { value: 3, prev: Link::Extra(2), next: Link::Entry(4) },
    ];

    unsafe {
        raw_links.0.as_mut().unwrap()[3] = Some(Links { next: 1, tail: 1 });
    }

    remove_extra_value(raw_links, &mut extra_values, 3);
}

#[test]
fn test_remove_extra_value_edge_case() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket::<u32>::default(); 128])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Entry(1) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(2) },
        ExtraValue { value: 2, prev: Link::Extra(1), next: Link::Extra(1) },
    ];

    unsafe {
        raw_links.0.as_mut().unwrap()[1] = Some(Links { next: 0, tail: 0 });
    }

    remove_extra_value(raw_links, &mut extra_values, 1);
}

#[test]
#[should_panic]
fn test_remove_extra_value_panic_index_out_of_bounds() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Bucket::<u32>::default(); 128])));
    let mut extra_values = vec![
        ExtraValue { value: 0, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Entry(1) },
    ];

    unsafe {
        raw_links.0.as_mut().unwrap()[1] = Some(Links { next: 2, tail: 2 });
    }

    remove_extra_value(raw_links, &mut extra_values, 3);
}

