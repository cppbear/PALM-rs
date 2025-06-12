// Answer 0

#[test]
fn test_remove_extra_value_case_1() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 }), Some(Bucket { next: 2, tail: 3 }), None, None, None, None, None, None, None, None])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 1, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 2, prev: Link::Entry(1), next: Link::Entry(0) },
        ExtraValue { value: 3, prev: Link::Extra(0), next: Link::Extra(1) },
        ExtraValue { value: 4, prev: Link::Entry(2), next: Link::Extra(2) },
        ExtraValue { value: 5, prev: Link::Entry(3), next: Link::Extra(3) },
        ExtraValue { value: 6, prev: Link::Extra(4), next: Link::Entry(4) },
    ];
    
    let idx = 1;
    let _result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_2() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 1 }), Some(Bucket { next: 2, tail: 3 }), None, None, None, None, None, None, None, None])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 10, prev: Link::Entry(1), next: Link::Extra(2) },
        ExtraValue { value: 20, prev: Link::Entry(2), next: Link::Extra(3) },
        ExtraValue { value: 30, prev: Link::Extra(3), next: Link::Extra(4) },
        ExtraValue { value: 40, prev: Link::Entry(3), next: Link::Entry(5) },
        ExtraValue { value: 50, prev: Link::Extra(4), next: Link::Extra(5) },
        ExtraValue { value: 60, prev: Link::Entry(4), next: Link::Entry(6) },
    ];
    
    let idx = 4;
    let _result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_3() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 2 }), Some(Bucket { next: 2, tail: 1 }), Some(Bucket { next: 0, tail: 2 })])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 100, prev: Link::Entry(0), next: Link::Extra(1) },
        ExtraValue { value: 200, prev: Link::Entry(1), next: Link::Extra(0) },
        ExtraValue { value: 300, prev: Link::Entry(2), next: Link::Extra(2) },
        ExtraValue { value: 400, prev: Link::Extra(1), next: Link::Entry(3) },
        ExtraValue { value: 500, prev: Link::Entry(3), next: Link::Extra(4) },
        ExtraValue { value: 600, prev: Link::Extra(5), next: Link::Extra(1) },
    ];
    
    let idx = 3;
    let _result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_4() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 1, tail: 0 }), Some(Bucket { next: 0, tail: 1 }), Some(Bucket { next: 1, tail: 0 }), None])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 7, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 8, prev: Link::Extra(1), next: Link::Extra(2) },
        ExtraValue { value: 9, prev: Link::Entry(2), next: Link::Extra(3) },
        ExtraValue { value: 10, prev: Link::Entry(1), next: Link::Extra(4) },
        ExtraValue { value: 11, prev: Link::Entry(3), next: Link::Entry(5) },
        ExtraValue { value: 12, prev: Link::Extra(4), next: Link::Entry(6) },
    ];
    
    let idx = 4;
    let _result = remove_extra_value(raw_links, &mut extra_values, idx);
}

#[test]
fn test_remove_extra_value_case_5() {
    let mut raw_links = RawLinks(Box::into_raw(Box::new([Some(Bucket { next: 0, tail: 2 }), Some(Bucket { next: 2, tail: 0 }), Some(Bucket { next: 0, tail: 1 }), None, None])));
    let mut extra_values: Vec<ExtraValue<u32>> = vec![
        ExtraValue { value: 13, prev: Link::Entry(0), next: Link::Entry(1) },
        ExtraValue { value: 14, prev: Link::Extra(0), next: Link::Entry(2) },
        ExtraValue { value: 15, prev: Link::Entry(2), next: Link::Extra(3) },
        ExtraValue { value: 16, prev: Link::Extra(3), next: Link::Entry(4) },
        ExtraValue { value: 17, prev: Link::Entry(3), next: Link::Extra(4) },
        ExtraValue { value: 18, prev: Link::Entry(2), next: Link::Extra(5) },
    ];
    
    let idx = 2;
    let _result = remove_extra_value(raw_links, &mut extra_values, idx);
}

