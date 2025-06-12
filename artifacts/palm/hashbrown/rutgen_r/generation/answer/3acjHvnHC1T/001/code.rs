// Answer 0

#[test]
fn test_values_mut_with_multiple_elements() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let values_mut = map.values_mut();
    for val in values_mut {
        *val += 10;
    }

    assert_eq!(map.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [11, 12, 13]);
}

#[test]
fn test_values_mut_with_single_element() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("x", 5);

    {
        let values_mut = map.values_mut();
        for val in values_mut {
            *val += 2;
        }
    }

    assert_eq!(map.len(), 1);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    assert_eq!(vec, [7]);
}

#[test]
fn test_values_mut_with_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    
    let values_mut = map.values_mut();
    for val in values_mut {
        // We expect this loop to never run since the map is empty
        *val += 1; // This line should not panic, but the loop should not execute
    }

    assert_eq!(map.len(), 0);
}

#[should_panic]
#[test]
fn test_values_mut_panic_on_detached_reference() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    {
        let mut values_mut = map.values_mut();
        let first = values_mut.next().unwrap();

        // Attempt to create a reference that outlives the borrow scope
        std::mem::drop(values_mut);
        *first += 10; // This should panic on runtime due to dereferencing after the borrow
    }
}

