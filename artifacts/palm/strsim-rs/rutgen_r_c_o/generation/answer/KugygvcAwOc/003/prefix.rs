// Answer 0

#[test]
fn test_lookup_existing_key() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1000, value: 10 }, 
                       GrowingHashmapMapElemChar::default(); 15]),
    };
    let index = hashmap.lookup(1000);
}

#[test]
fn test_lookup_non_existing_key_with_collision() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 2000, value: 20 }, 
                       GrowingHashmapMapElemChar::default(); 15]),
    };
    let index = hashmap.lookup(3000);
}

#[test]
fn test_lookup_non_default_value() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 2,
        fill: 2,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar { key: 4000, value: 30 }, 
                       GrowingHashmapMapElemChar { key: 1000, value: 10 }, 
                       GrowingHashmapMapElemChar::default(); 15]),
    };
    let index = hashmap.lookup(1000);
}

#[test]
fn test_lookup_returning_default() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 15,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 15]),
    };
    let index = hashmap.lookup(5000);
}

#[test]
fn test_lookup_panic_if_not_allocated() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let index = hashmap.lookup(1000);
}

