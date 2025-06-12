// Answer 0

#[test]
fn test_lookup_key_found_with_default_value() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 16]),
    };
    hashmap.map.as_mut().unwrap()[hashmap.lookup(0)].value = 5;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(0)].key = 0; 
    let _ = hashmap.lookup(0);
}

#[test]
fn test_lookup_key_found_value_non_default() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 16]),
    };
    hashmap.map.as_mut().unwrap()[hashmap.lookup(1)].value = 10;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(1)].key = 1; 
    let _ = hashmap.lookup(1);
}

#[test]
fn test_lookup_key_with_collision_resolution() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 31,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 32]),
    };
    hashmap.map.as_mut().unwrap()[hashmap.lookup(10)].value = 20;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(10)].key = 10; 
    let _ = hashmap.lookup(10);
}

#[test]
fn test_lookup_multiple_keys_found() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 31,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 32]),
    };
    hashmap.map.as_mut().unwrap()[hashmap.lookup(5)].value = 15;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(5)].key = 5; 
    hashmap.map.as_mut().unwrap()[hashmap.lookup(20)].value = 25;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(20)].key = 20; 
    let _ = hashmap.lookup(5);
    let _ = hashmap.lookup(20);
}

#[test]
fn test_lookup_non_default_values_sequentially() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 4,
        fill: 4,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 16]),
    };
    hashmap.map.as_mut().unwrap()[hashmap.lookup(2)].value = 30;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(2)].key = 2; 
    hashmap.map.as_mut().unwrap()[hashmap.lookup(7)].value = 35;
    hashmap.map.as_mut().unwrap()[hashmap.lookup(7)].key = 7; 
    let _ = hashmap.lookup(2);
    let _ = hashmap.lookup(7);
}

