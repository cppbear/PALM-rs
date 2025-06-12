// Answer 0

#[derive(Debug)]
struct AbsentEntry;

impl std::fmt::Display for AbsentEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AbsentEntry")
    }
}

#[test]
fn test_fmt() {
    let entry = AbsentEntry;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", entry);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "AbsentEntry");
}

