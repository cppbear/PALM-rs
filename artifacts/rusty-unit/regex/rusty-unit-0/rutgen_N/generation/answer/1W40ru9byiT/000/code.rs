// Answer 0

#[derive(Default)]
struct Locations(Vec<Slot>);

#[derive(Default)]
struct Slot;

#[test]
fn test_as_slots() {
    let mut locations = Locations(vec![Slot::default(), Slot::default()]);
    let slots: &mut [Slot] = as_slots(&mut locations);
    assert_eq!(slots.len(), 2);
}

#[test]
fn test_as_slots_empty() {
    let mut locations = Locations(vec![]);
    let slots: &mut [Slot] = as_slots(&mut locations);
    assert_eq!(slots.len(), 0);
}

