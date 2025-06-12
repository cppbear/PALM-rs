// Answer 0

#[test]
fn test_grow_case_1() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 5,
        fill: 5,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar { key: 5, value: 50 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(10);
}

#[test]
fn test_grow_case_2() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(5);
}

#[test]
fn test_grow_case_3() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 15 },
            GrowingHashmapMapElemChar { key: 2, value: 25 },
            GrowingHashmapMapElemChar { key: 3, value: 35 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(6);
}

#[test]
fn test_grow_case_4() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 2,
        fill: 2,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 100 },
            GrowingHashmapMapElemChar { key: 2, value: 100 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(10);
}

#[test]
fn test_grow_case_5() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 31,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(2);
}

