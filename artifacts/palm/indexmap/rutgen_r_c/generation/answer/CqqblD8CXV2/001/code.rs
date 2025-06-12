// Answer 0

#[test]
fn test_slice_fmt_empty() {
    let slice: Slice<i32, i32> = Slice { entries: [] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", slice);
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
fn test_slice_fmt_single_entry() {
    let entry = Bucket { hash: HashValue(1), key: 1, value: 100 };
    let slice = Slice { entries: [entry] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", slice);
    assert!(result.is_ok());
    assert_eq!(output, "[1: 100]");
}

#[test]
fn test_slice_fmt_multiple_entries() {
    let entry1 = Bucket { hash: HashValue(1), key: 1, value: 100 };
    let entry2 = Bucket { hash: HashValue(2), key: 2, value: 200 };
    let slice = Slice { entries: [entry1, entry2] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", slice);
    assert!(result.is_ok());
    assert_eq!(output, "[1: 100, 2: 200]");
}

#[test]
fn test_slice_fmt_multiple_entries_with_same_keys() {
    let entry1 = Bucket { hash: HashValue(1), key: 1, value: 100 };
    let entry2 = Bucket { hash: HashValue(2), key: 1, value: 200 };
    let slice = Slice { entries: [entry1, entry2] };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", slice);
    assert!(result.is_ok());
    assert_eq!(output, "[1: 100, 1: 200]");
}

