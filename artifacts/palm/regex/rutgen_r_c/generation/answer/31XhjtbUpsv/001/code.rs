// Answer 0

#[test]
fn test_iter_empty_locations() {
    let locations = Locations(Vec::new());
    let iter = locations.iter();
    assert_eq!(iter.idx, 0);
}

#[test]
fn test_iter_non_empty_locations() {
    let locations = Locations(vec![Some(0), None, Some(2)]);
    let iter = locations.iter();
    assert_eq!(iter.idx, 0);
}

#[test]
fn test_iter_len() {
    let locations = Locations(vec![Some(0), None, Some(2), None]);
    let iter = locations.iter();
    assert_eq!(iter.idx, 0);
    assert_eq!(locations.len(), 4);
}

