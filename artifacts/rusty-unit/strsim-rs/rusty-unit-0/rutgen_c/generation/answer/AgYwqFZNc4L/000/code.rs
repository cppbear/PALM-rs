// Answer 0

#[test]
fn test_get_mut_new_entry() {
    struct TestValueType {
        data: usize,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let key = 5;
    let value = hashmap.get_mut(key);
    value.data = 42;

    assert_eq!(hashmap.get(5).data, 42);
}

#[test]
fn test_get_mut_existing_entry() {
    struct TestValueType {
        data: usize,
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let key = 10;
    {
        let value = hashmap.get_mut(key);
        value.data = 31;
    }

    let value = hashmap.get_mut(key);
    value.data += 11;

    assert_eq!(hashmap.get(10).data, 42);
}

#[test]
fn test_get_mut_growth() {
    struct TestValueType {
        data: usize,
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    for i in 0..10 {
        let value = hashmap.get_mut(i);
        value.data = i as usize;
    }

    for i in 0..10 {
        assert_eq!(hashmap.get(i).data, i);
    }
}

#[test]
fn test_get_mut_boundary_conditions() {
    struct TestValueType {
        data: usize,
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let value = hashmap.get_mut(0);
    value.data = 1;
    assert_eq!(hashmap.get(0).data, 1);

    let value = hashmap.get_mut(7);
    value.data = 2;
    assert_eq!(hashmap.get(7).data, 2);
}

