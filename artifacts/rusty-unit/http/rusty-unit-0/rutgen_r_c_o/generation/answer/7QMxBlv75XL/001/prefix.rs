// Answer 0

#[test]
fn test_remove_all_extra_values_case1() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(10);
    // Prepare mock extra values linked in a chain with indexes
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value1"),
        prev: Link::None,
        next: Link::Extra(1),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value2"),
        prev: Link::Extra(0),
        next: Link::Extra(2),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("value3"),
        prev: Link::Extra(1),
        next: Link::None,
    });
    
    // Starting head value that references the first extra value
    header_map.remove_all_extra_values(0);
}

#[test]
fn test_remove_all_extra_values_case2() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(5);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("valueA"),
        prev: Link::None,
        next: Link::Extra(1),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("valueB"),
        prev: Link::Extra(0),
        next: Link::Extra(2),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("valueC"),
        prev: Link::Extra(1),
        next: Link::Extra(3),
    });
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("valueD"),
        prev: Link::Extra(2),
        next: Link::None,
    });

    // Starting head value referencing the first extra value
    header_map.remove_all_extra_values(0);
}

#[test]
fn test_remove_all_extra_values_empty_chain() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(1);
    // No extra values to remove, starting at head 0
    header_map.remove_all_extra_values(0);
}

#[test]
fn test_remove_all_extra_values_single_value() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(1);
    header_map.extra_values.push(ExtraValue {
        value: HeaderValue::from("only_value"),
        prev: Link::None,
        next: Link::None,
    });

    // Head starts at the single extra value's index
    header_map.remove_all_extra_values(0);
}

#[test]
fn test_remove_all_extra_values_large_chain() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(100);
    for i in 0..100 {
        header_map.extra_values.push(ExtraValue {
            value: HeaderValue::from(format!("value{}", i)),
            prev: if i == 0 { Link::None } else { Link::Extra(i - 1) },
            next: if i < 99 { Link::Extra(i + 1) } else { Link::None },
        });
    }
    
    // Start at the head value of the first extra value
    header_map.remove_all_extra_values(0);
}

