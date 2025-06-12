// Answer 0

#[test]
fn test_get_or_insert_with_existing_value() {
    let mut set: HashSet<String> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable {
                table: RawTableInner::new(),
                alloc: Global,
                marker: PhantomData,
            },
        },
    };
    
    set.insert("cat".to_owned());
    set.insert("dog".to_owned());
    
    let value = set.get_or_insert_with("cat", |s| s.to_owned());
    assert_eq!(value, "cat");
    assert_eq!(set.map.len(), 2);
}

#[test]
fn test_get_or_insert_with_inserting_new_value() {
    let mut set: HashSet<String> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable {
                table: RawTableInner::new(),
                alloc: Global,
                marker: PhantomData,
            },
        },
    };
    
    set.insert("cat".to_owned());
    set.insert("dog".to_owned());
    
    let value = set.get_or_insert_with("fish", |s| s.to_owned());
    assert_eq!(value, "fish");
    assert_eq!(set.map.len(), 3);
}

#[should_panic(expected = "new value is not equivalent")]
#[test]
fn test_get_or_insert_with_panic_on_equivalence_fail() {
    let mut set: HashSet<String> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder::new(),
            table: RawTable {
                table: RawTableInner::new(),
                alloc: Global,
                marker: PhantomData,
            },
        },
    };
    
    set.insert("rust".to_owned());
    
    set.get_or_insert_with("rust", |_| String::new());
}

