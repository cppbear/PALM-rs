// Answer 0

#[test]
fn test_splice_fmt_empty() {
    use std::collections::hash_map::RandomState;
    
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let drain: Vec<Bucket<i32, i32>> = vec![];
    let replace_with = vec![(1, 2)].into_iter();

    let splice = Splice {
        map: &mut map,
        tail: IndexMapCore {
            indices: vec![],
            entries: vec![],
        },
        drain: drain.into_iter(),
        replace_with,
    };

    let mut output = vec![];
    let result = fmt::write(&mut output, |f| splice.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Splice { drain: [], replace_with: [1, 2] }");
}

#[test]
fn test_splice_fmt_with_drain() {
    use std::collections::hash_map::RandomState;
    
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let drain = vec![
        Bucket { hash: 1, key: 1, value: 10 },
        Bucket { hash: 2, key: 2, value: 20 },
    ];
    let replace_with = vec![(3, 30), (4, 40)].into_iter();

    let splice = Splice {
        map: &mut map,
        tail: IndexMapCore {
            indices: vec![],
            entries: vec![],
        },
        drain: drain.into_iter(),
        replace_with,
    };

    let mut output = vec![];
    let result = fmt::write(&mut output, |f| splice.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Splice { drain: [Bucket { hash: 1, key: 1, value: 10 }, Bucket { hash: 2, key: 2, value: 20 }], replace_with: [3, 30], [4, 40] }");
}

#[test]
fn test_splice_fmt_with_varied_data() {
    use std::collections::hash_map::RandomState;
    
    let mut map: IndexMap<String, String, RandomState> = IndexMap::default();
    let drain = vec![
        Bucket { hash: 3, key: "apple".to_string(), value: "red".to_string() },
        Bucket { hash: 4, key: "banana".to_string(), value: "yellow".to_string() },
        Bucket { hash: 5, key: "cherry".to_string(), value: "dark red".to_string() },
    ];
    let replace_with = vec![("date".to_string(), "brown".to_string())].into_iter();

    let splice = Splice {
        map: &mut map,
        tail: IndexMapCore {
            indices: vec![],
            entries: vec![],
        },
        drain: drain.into_iter(),
        replace_with,
    };

    let mut output = vec![];
    let result = fmt::write(&mut output, |f| splice.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Splice { drain: [Bucket { hash: 3, key: \"apple\", value: \"red\" }, Bucket { hash: 4, key: \"banana\", value: \"yellow\" }, Bucket { hash: 5, key: \"cherry\", value: \"dark red\" }], replace_with: [\"date\", \"brown\"] }");
}

