// Answer 0

#[test]
fn test_get_mut_with_initial_allocation() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let _value = hashmap.get_mut(1);
}

#[test]
fn test_get_mut_with_filled_value() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let _value1 = hashmap.get_mut(2);
    *hashmap.get_mut(2) = 5;
    let _value2 = hashmap.get_mut(2);
}

#[test]
fn test_get_mut_with_growth_trigger() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let _value1 = hashmap.get_mut(3);
    let _value2 = hashmap.get_mut(4);
    *hashmap.get_mut(3) = 10;
    *hashmap.get_mut(4) = 15;
}

#[test]
fn test_get_mut_with_preallocated_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 100 },
            GrowingHashmapMapElemChar { key: 2, value: 200 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let _value = hashmap.get_mut(1);
}

