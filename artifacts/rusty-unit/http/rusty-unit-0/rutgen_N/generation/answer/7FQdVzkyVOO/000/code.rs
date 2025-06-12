// Answer 0

#[test]
fn test_iter_mut_with_single_value() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "single-host".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        for e in e.iter_mut() {
            e.push_str("-modified");
        }
    }

    let mut values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!(&"single-host-modified", i.next().unwrap());
    assert!(i.next().is_none());
}

#[test]
fn test_iter_mut_with_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "first-host".to_string());
    map.append(HOST, "second-host".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        for e in e.iter_mut() {
            e.push_str("-updated");
        }
    }

    let mut values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!(&"first-host-updated", i.next().unwrap());
    assert_eq!(&"second-host-updated", i.next().unwrap());
    assert!(i.next().is_none());
}

#[test]
fn test_iter_mut_empty_entry() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();

    if let Entry::Vacant(e) = map.entry("host") {
        e.insert("new-host".to_string());
    }

    let mut values = map.get_all("host");
    let mut i = values.iter();
    assert_eq!(&"new-host", i.next().unwrap());
    assert!(i.next().is_none());
}

