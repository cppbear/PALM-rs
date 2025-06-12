// Answer 0

#[test]
fn test_expand_bytes_with_number_ref() {
    use re_bytes::Captures;
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = b"example";
    let locations = vec![(0, 7)]; // The full range of the text
    let group_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs: Locations::from(locations),
        named_groups: group_map,
    };

    let mut dst = Vec::new();
    let replacement = b"match $0";
    expand_bytes(&caps, replacement, &mut dst);
    assert_eq!(dst, b"match example");
}

#[test]
fn test_expand_bytes_with_named_ref() {
    use re_bytes::Captures;
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = b"hello";
    let locations = vec![(0, 5)]; // The full range of the text
    let mut group_map = HashMap::new();
    group_map.insert("greeting".to_string(), 0);
    let caps = Captures {
        text,
        locs: Locations::from(locations),
        named_groups: Arc::new(group_map),
    };

    let mut dst = Vec::new();
    let replacement = b"prefix ${greeting}";
    expand_bytes(&caps, replacement, &mut dst);
    assert_eq!(dst, b"prefix hello");
}

#[test]
fn test_expand_bytes_with_double_dollar() {
    use re_bytes::Captures;
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = b"data";
    let locations = vec![(0, 4)]; // The full range of the text
    let group_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs: Locations::from(locations),
        named_groups: group_map,
    };

    let mut dst = Vec::new();
    let replacement = b"price $$${0}";
    expand_bytes(&caps, replacement, &mut dst);
    assert_eq!(dst, b"price $$data");
}

#[test]
fn test_expand_bytes_with_empty_replacement() {
    use re_bytes::Captures;
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = b"test";
    let locations = vec![(0, 4)];
    let group_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs: Locations::from(locations),
        named_groups: group_map,
    };

    let mut dst = Vec::new();
    let replacement = b"$";
    expand_bytes(&caps, replacement, &mut dst);
    assert_eq!(dst, b"test");
}

