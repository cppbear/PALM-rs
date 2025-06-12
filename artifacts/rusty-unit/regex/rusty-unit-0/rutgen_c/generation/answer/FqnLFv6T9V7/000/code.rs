// Answer 0

#[test]
fn test_locations_len_non_empty() {
    let locations = Locations(vec![Some(0), Some(1)]);
    assert_eq!(locations.len(), 1);
}

#[test]
fn test_locations_len_empty() {
    let locations = Locations(vec![]);
    assert_eq!(locations.len(), 0);
}

#[test]
fn test_locations_len_with_multiple_groups() {
    let locations = Locations(vec![Some(0), Some(1), Some(2), Some(3)]);
    assert_eq!(locations.len(), 2);
}

