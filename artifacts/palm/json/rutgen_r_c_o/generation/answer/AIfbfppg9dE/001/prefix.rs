// Answer 0

#[test]
fn test_map_access_creation_empty_scratch() {
    let mut deserializer = Deserializer {
        read: Vec::new(),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _map_access = MapAccess::new(&mut deserializer);
}

#[test]
fn test_map_access_creation_with_small_scratch() {
    let mut deserializer = Deserializer {
        read: Vec::new(),
        scratch: vec![1, 2, 3],
        remaining_depth: 0,
    };
    let _map_access = MapAccess::new(&mut deserializer);
}

#[test]
fn test_map_access_creation_with_exact_scratch_size() {
    let mut deserializer = Deserializer {
        read: Vec::new(),
        scratch: vec![0; 256],
        remaining_depth: 0,
    };
    let _map_access = MapAccess::new(&mut deserializer);
}

#[test]
#[should_panic]
fn test_map_access_creation_with_remaining_depth_non_zero() {
    let mut deserializer = Deserializer {
        read: Vec::new(),
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    let _map_access = MapAccess::new(&mut deserializer);
}

