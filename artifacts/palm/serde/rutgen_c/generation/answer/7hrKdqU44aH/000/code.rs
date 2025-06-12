// Answer 0

#[test]
fn test_split() {
    struct PairStruct(i32, String);
    
    let pair = PairStruct(42, String::from("Hello"));
    let (first, second) = pair.split();

    assert_eq!(first, 42);
    assert_eq!(second, "Hello");
}

#[test]
fn test_split_different_types() {
    struct PairStruct(f64, bool);
    
    let pair = PairStruct(3.14, true);
    let (first, second) = pair.split();

    assert_eq!(first, 3.14);
    assert_eq!(second, true);
}

#[test]
fn test_split_empty_string() {
    struct PairStruct(char, String);
    
    let pair = PairStruct('a', String::from(""));
    let (first, second) = pair.split();

    assert_eq!(first, 'a');
    assert_eq!(second, "");
}

