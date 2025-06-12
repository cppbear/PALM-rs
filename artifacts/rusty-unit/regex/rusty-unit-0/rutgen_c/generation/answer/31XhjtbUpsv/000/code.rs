// Answer 0

#[test]
fn test_iter_initialization() {
    let locations = Locations(vec![Some(0), Some(5), None, Some(10)]);
    let mut iter = locations.iter();
    assert_eq!(iter.idx, 0);
    assert_eq!(iter.locs.0.len(), 4);
}

#[test]
fn test_iter_locations_length() {
    let locations = Locations(vec![Some(0), Some(2), Some(4)]);
    let iter = locations.iter();
    assert_eq!(iter.locs.len(), 3);
}

#[test]
fn test_iter_with_empty_locations() {
    let locations = Locations(Vec::new());
    let iter = locations.iter();
    assert_eq!(iter.locs.len(), 0);
    assert_eq!(iter.idx, 0);
}

