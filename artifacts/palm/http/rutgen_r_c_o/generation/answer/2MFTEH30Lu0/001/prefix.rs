// Answer 0

#[test]
fn test_remove_extra_value_valid_index() {
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value1"),
        prev: Link::Extra(0),
        next: Link::Extra(1),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value2"),
        prev: Link::Extra(1),
        next: Link::Extra(2),
    });

    let result = header_map.remove_extra_value(0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_remove_extra_value_out_of_bounds_high_index() {
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value1"),
        prev: Link::Extra(0),
        next: Link::Extra(1),
    });

    let result = header_map.remove_extra_value(2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_remove_extra_value_out_of_bounds_low_index() {
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(1);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value1"),
        prev: Link::Extra(0),
        next: Link::Extra(1),
    });

    let result = header_map.remove_extra_value(usize::MAX);
}

#[test]
fn test_remove_extra_value_all_indices() {
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(3);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value1"),
        prev: Link::Entry(0),
        next: Link::Extra(1),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value2"),
        prev: Link::Extra(0),
        next: Link::Entry(2),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value3"),
        prev: Link::Entry(1),
        next: Link::Extra(2),
    });

    let result1 = header_map.remove_extra_value(0);
    let result2 = header_map.remove_extra_value(1);
}

