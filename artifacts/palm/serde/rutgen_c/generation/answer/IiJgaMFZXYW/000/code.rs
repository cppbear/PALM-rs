// Answer 0

#[test]
fn test_map_as_enum() {
    struct TestMap {
        data: i32,
    }

    let test_map = TestMap { data: 42 };
    let result = map_as_enum(test_map);
    
    assert_eq!(result.map.data, 42);
}

#[test]
fn test_map_as_enum_with_different_type() {
    struct AnotherMap {
        value: String,
    }

    let another_map = AnotherMap { value: String::from("Hello") };
    let result = map_as_enum(another_map);
    
    assert_eq!(result.map.value, "Hello");
}

