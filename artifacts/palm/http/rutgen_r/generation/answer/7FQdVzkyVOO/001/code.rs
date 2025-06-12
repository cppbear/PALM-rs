// Answer 0

#[test]
fn test_iter_mut_single_value() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "single-value".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        for e in e.iter_mut() {
            e.push_str("-modified");
        }
    }

    let mut values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!(&"single-value-modified", i.next().unwrap());
}

#[test]
fn test_iter_mut_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "first-value".to_string());
    map.append(HOST, "second-value".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        for e in e.iter_mut() {
            e.push_str("-modified");
        }
    }

    let mut values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!(&"first-value-modified", i.next().unwrap());
    assert_eq!(&"second-value-modified", i.next().unwrap());
}

#[test]
fn test_iter_mut_no_entry() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();

    if let Entry::Vacant(_) = map.entry("host") {
        // No operation needed as we expect no entries
    }

    let values = map.get_all("host");
    assert!(values.is_empty());
}

#[should_panic]
#[test]
fn test_iter_mut_panic_on_non_existent_entry() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();

    if let Entry::Occupied(mut e) = map.entry("host") {
        for e in e.iter_mut() {
            e.push_str("-no-op");
        }
    }

    // This should not panic as we expect no entry here
    let mut values = map.get_all("host");
    let mut i = values.iter();
    // Attempting to unwrap should panic since it's empty
    assert_eq!(i.next().unwrap(), "");
}

