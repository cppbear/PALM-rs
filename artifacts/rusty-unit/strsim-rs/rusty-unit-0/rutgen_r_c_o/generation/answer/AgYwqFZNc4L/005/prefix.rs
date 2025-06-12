// Answer 0

#[test]
fn test_get_mut_initial_insert() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let key: u32 = 5;
    let value = hashmap.get_mut(key);
}

#[test]
fn test_get_mut_overwrite_existing_value() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let key: u32 = 3;
    let value1 = hashmap.get_mut(key);
    *value1 = 10;
    let value2 = hashmap.get_mut(key);
}

#[test]
fn test_get_mut_fill_trigger_grow() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(); 8
        ]),
    };
    hashmap.fill = 5; // Manually set fill to a state
    let key: u32 = 2;
    let value = hashmap.get_mut(key);
}

#[test]
fn test_get_mut_resize_on_fill() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(); 4
        ]),
    };
    hashmap.fill = 3; // at max capacity to trigger resize
    let key: u32 = 1;
    let value = hashmap.get_mut(key);
}

#[test]
fn test_get_mut_no_grow_needed() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(); 8
        ]),
    };
    let key: u32 = 6;
    let value = hashmap.get_mut(key);
}

