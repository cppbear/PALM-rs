// Answer 0

#[test]
fn test_pos_valid_capture_group() {
    let locations = Locations(vec![Some(0), Some(5), Some(10), Some(15)]);
    assert_eq!(locations.pos(1), Some((5, 10)));
}

#[test]
fn test_pos_invalid_capture_group() {
    let locations = Locations(vec![Some(0), Some(5)]);
    assert_eq!(locations.pos(2), None);
}

#[test]
fn test_pos_no_match() {
    let locations = Locations(vec![Some(0), None, Some(10), None]);
    assert_eq!(locations.pos(1), None);
}

#[test]
fn test_pos_empty_locations() {
    let locations = Locations(vec![]);
    assert_eq!(locations.pos(0), None);
}

