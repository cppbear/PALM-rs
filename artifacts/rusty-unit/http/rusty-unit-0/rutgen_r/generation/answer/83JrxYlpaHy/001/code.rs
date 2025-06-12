// Answer 0

#[test]
fn test_get_first_value_success() {
    use http::header::{HeaderMap, Entry, HOST};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "example.com".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        assert_eq!(e.get(), &"example.com");
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant.");
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_panic_on_empty_entry() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();

    if let Entry::Occupied(e) = map.entry("missing") {
        let _ = e.get(); // This should panic
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant.");
    }
}

#[test]
fn test_get_first_value_after_append() {
    use http::header::{HeaderMap, Entry, HOST};
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "first.example.com".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        assert_eq!(e.get(), &"first.example.com");
        
        e.append("second.example.com".parse().unwrap());
        
        assert_eq!(e.get(), &"first.example.com");
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant.");
    }
}

