// Answer 0

#[test]
fn test_intersection_debug_fmt_with_empty_set() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashMap;

    struct MySet;
    impl BuildHasher for MySet {
        type Hasher = BuildHasherDefault<HashMap<i32, ()>>;
        fn build_hasher(&self) -> Self::Hasher {
            BuildHasherDefault::default()
        }
    }

    let empty_set: IndexSet<i32, MySet> = IndexSet { map: IndexMap::new() };
    let intersection = Intersection { iter: Iter { iter: empty_set.map.iter() }, other: &empty_set };
    
    let result = format!("{:?}", intersection);
    assert_eq!(result, "[]");
}

#[test]
fn test_intersection_debug_fmt_with_non_empty_set() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashMap;

    struct MySet;
    impl BuildHasher for MySet {
        type Hasher = BuildHasherDefault<HashMap<i32, ()>>;
        fn build_hasher(&self) -> Self::Hasher {
            BuildHasherDefault::default()
        }
    }

    let mut non_empty_set: IndexSet<i32, MySet> = IndexSet { map: IndexMap::new() };
    non_empty_set.map.insert(1, ());
    non_empty_set.map.insert(2, ());

    let intersection = Intersection { iter: Iter { iter: non_empty_set.map.iter() }, other: &non_empty_set };
    
    let result = format!("{:?}", intersection);
    assert_eq!(result, "[1, 2]");
}

#[should_panic]
#[test]
fn test_intersection_debug_fmt_with_panic_condition() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashMap;

    struct MySet;
    impl BuildHasher for MySet {
        type Hasher = BuildHasherDefault<HashMap<i32, ()>>;
        fn build_hasher(&self) -> Self::Hasher {
            BuildHasherDefault::default()
        }
    }

    let non_empty_set: IndexSet<i32, MySet> = IndexSet { map: IndexMap::new() };
    let intersection = Intersection { iter: Iter { iter: non_empty_set.map.iter() }, other: &non_empty_set };

    // The following should panic as we attempt to access invalid data
    let _ = format!("{:?}", intersection.clone());
}

