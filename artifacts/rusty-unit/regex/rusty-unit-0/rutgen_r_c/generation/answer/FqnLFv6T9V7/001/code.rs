// Answer 0

#[test]
fn test_len_with_even_slots() {
    let locations = Locations(vec![Some(0), Some(1), Some(2), Some(3)]);
    assert_eq!(locations.len(), 2);
}

#[test]
fn test_len_with_odd_slots() {
    let locations = Locations(vec![Some(0), Some(1), Some(2)]);
    assert_eq!(locations.len(), 1);
}

#[test]
fn test_len_with_no_slots() {
    let locations = Locations(vec![]);
    assert_eq!(locations.len(), 0);
}

#[test]
fn test_len_with_nil_slots() {
    let locations = Locations(vec![None, None, None, None]);
    assert_eq!(locations.len(), 2);
}

#[test]
fn test_len_with_mixed_slots() {
    let locations = Locations(vec![Some(0), None, Some(2), None]);
    assert_eq!(locations.len(), 2);
}

