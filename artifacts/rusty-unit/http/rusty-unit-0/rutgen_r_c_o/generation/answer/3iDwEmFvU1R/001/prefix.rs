// Answer 0

#[test]
fn test_try_append_small_header_name() {
    let mut map = HeaderMap::new();
    let header_name = "X-Small-Header";
    let value = "Value1";
    header_name.try_append(&mut map, value);
}

#[test]
fn test_try_append_large_header_name() {
    let mut map = HeaderMap::new();
    let header_name = "X-Large-Header-Name-That-Is-Exactly-255-Characters-Long-And-Continues-To-Expand-Until-It-Reaches-The-Character-Limit-Without-Any-Additional-Text-Just-A-Header-In-The-End-Yeah-That-Should-Be-Fine";
    let value = "Value2";
    header_name.try_append(&mut map, value);
}

#[test]
fn test_try_append_empty_map() {
    let mut map = HeaderMap::new();
    let header_name = "X-Empty-Map";
    let value = 42;
    header_name.try_append(&mut map, value);
}

#[test]
fn test_try_append_map_reaches_capacity() {
    let mut map = HeaderMap::new();
    let header_name = "X-Mid-Capacity-Header";
    let value = "Value3";

    for i in 0..1_000_000 {
        let entry_name = format!("Header-{}", i);
        entry_name.try_append(&mut map, i);
    }

    let result = header_name.try_append(&mut map, value);
    assert!(result.is_err());
}

#[test]
fn test_try_append_various_types() {
    let mut map = HeaderMap::new();
    
    let header_name = "X-Various-Types";
    
    let value_str = "StringValue";
    header_name.try_append(&mut map, value_str);

    let value_int = 100;
    header_name.try_append(&mut map, value_int);

    let value_float = 99.99;
    header_name.try_append(&mut map, value_float);

    let value_boxed = Box::new("BoxedString");
    header_name.try_append(&mut map, value_boxed);
}

