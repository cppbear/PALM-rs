// Answer 0

#[test]
fn test_to_vec_non_serializable() {
    struct NonSerializable;
    let data = NonSerializable;
    let result = to_vec(&data);
}

#[test]
fn test_to_vec_map_with_non_string_keys() {
    use std::collections::HashMap;
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("value"));
    let result = to_vec(&map);
}

#[test]
fn test_to_vec_large_nested_structure() {
    #[derive(Serialize)]
    struct LargeNested {
        data: Vec<Vec<i32>>,
    }

    let data = LargeNested { data: vec![vec![1; 1000]; 1000] };
    let result = to_vec(&data);
}

#[test]
fn test_to_vec_circular_reference() {
    #[derive(Serialize)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }
    
    let mut node_a = Node { value: 1, next: None };
    let node_b = Node { value: 2, next: Some(Box::new(node_a)) };
    node_a.next = Some(Box::new(node_b));
    let result = to_vec(&node_a);
}

