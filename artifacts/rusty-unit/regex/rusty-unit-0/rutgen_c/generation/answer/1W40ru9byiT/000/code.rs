// Answer 0

#[test]
fn test_as_slots_empty() {
    let mut locations = Locations(vec![]);
    let slots = as_slots(&mut locations);
    assert_eq!(slots.len(), 0);
}

#[test]
fn test_as_slots_single_element() {
    let mut locations = Locations(vec![Some(1)]);
    let slots = as_slots(&mut locations);
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0], Some(1));
}

#[test]
fn test_as_slots_multiple_elements() {
    let mut locations = Locations(vec![Some(1), None, Some(3)]);
    let slots = as_slots(&mut locations);
    assert_eq!(slots.len(), 3);
    assert_eq!(slots[0], Some(1));
    assert_eq!(slots[1], None);
    assert_eq!(slots[2], Some(3));
}

#[test]
fn test_as_slots_mutability() {
    let mut locations = Locations(vec![None]);
    {
        let slots = as_slots(&mut locations);
        slots[0] = Some(42);
    }
    assert_eq!(locations.0[0], Some(42));
}

