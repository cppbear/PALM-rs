// Answer 0

#[test]
fn test_fmt_debug_with_single_bucket() {
    let key = "key1";
    let value = "value1";
    let bucket = Bucket { hash: 0, key, value };
    let iter = IntoIter { iter: vec![bucket].into_iter() };
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| iter.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "[\"key1\": \"value1\"]");
}

#[test]
fn test_fmt_debug_with_multiple_buckets() {
    let buckets = vec![
        Bucket { hash: 1, key: "key1", value: "value1" },
        Bucket { hash: 2, key: "key2", value: "value2" },
    ];
    let iter = IntoIter { iter: buckets.into_iter() };
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| iter.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "[\"key1\": \"value1\", \"key2\": \"value2\"]");
}

#[test]
fn test_fmt_debug_with_no_buckets() {
    let iter = IntoIter { iter: Vec::<Bucket<_, _>>::new().into_iter() };
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| iter.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

