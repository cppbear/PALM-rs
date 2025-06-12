// Answer 0

#[test]
fn test_fmt_with_empty_drain_and_replace_with() {
    let mut map = IndexMap::new();
    let drain = vec![];
    let replace_with = vec![("key1", "value1")];
    let drain_iter = drain.into_iter();
    let splice = Splice { map: &mut map, tail: IndexMapCore::new(), drain: drain_iter, replace_with };
    let _ = fmt::format(format_args!("{:?}", splice));
}

#[test]
fn test_fmt_with_non_empty_drain_and_empty_replace_with() {
    let mut map = IndexMap::new();
    let drain = vec![Bucket { hash: 1, key: 1, value: 2 }, Bucket { hash: 2, key: 2, value: 3 }];
    let replace_with = vec![];
    let drain_iter = drain.into_iter();
    let splice = Splice { map: &mut map, tail: IndexMapCore::new(), drain: drain_iter, replace_with };
    let _ = fmt::format(format_args!("{:?}", splice));
}

#[test]
fn test_fmt_with_non_empty_drain_and_replace_with() {
    let mut map = IndexMap::new();
    let drain = vec![Bucket { hash: 1, key: 1, value: 2 }];
    let replace_with = vec![("key3", "value3")];
    let drain_iter = drain.into_iter();
    let splice = Splice { map: &mut map, tail: IndexMapCore::new(), drain: drain_iter, replace_with };
    let _ = fmt::format(format_args!("{:?}", splice));
}

#[test]
fn test_fmt_with_large_drain_and_replace_with() {
    let mut map = IndexMap::new();
    let drain = (1..51).map(|i| Bucket { hash: i as u64, key: i, value: i * 2 }).collect::<Vec<_>>();
    let replace_with = (1..51).map(|i| (i, i * 3)).collect::<Vec<_>>();
    let drain_iter = drain.into_iter();
    let splice = Splice { map: &mut map, tail: IndexMapCore::new(), drain: drain_iter, replace_with };
    let _ = fmt::format(format_args!("{:?}", splice));
}

#[test]
fn test_fmt_with_full_drain_and_replace_with() {
    let mut map = IndexMap::new();
    let drain = (1..51).map(|i| Bucket { hash: i as u64, key: i, value: i }).collect::<Vec<_>>();
    let replace_with = (51..101).map(|i| (i, i)).collect::<Vec<_>>();
    let drain_iter = drain.into_iter();
    let splice = Splice { map: &mut map, tail: IndexMapCore::new(), drain: drain_iter, replace_with };
    let _ = fmt::format(format_args!("{:?}", splice));
}

