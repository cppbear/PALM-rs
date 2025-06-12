// Answer 0

#[test]
fn test_fmt_debug_into_iter() {
    #[derive(Copy, Clone, Debug)]
    struct HashValue(u64);
    
    let buckets = vec![
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ];
    
    let iter = IntoIter {
        iter: buckets.into_iter().collect::<Vec<_>>().into_iter(),
    };
    
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    iter.fmt(formatter).unwrap();
    
    assert!(output.contains("10") && output.contains("a"));
    assert!(output.contains("20") && output.contains("b"));
}

#[test]
fn test_fmt_debug_empty_iter() {
    let iter = IntoIter {
        iter: Vec::new().into_iter(),
    };
    
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    iter.fmt(formatter).unwrap();
    
    assert!(output.is_empty());
}

