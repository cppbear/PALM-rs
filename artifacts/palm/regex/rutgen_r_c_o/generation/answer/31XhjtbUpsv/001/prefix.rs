// Answer 0

#[test]
fn test_iter_empty_locations() {
    let locations = Locations(Vec::new());
    let iter = locations.iter();
}

#[test]
fn test_iter_single_slot_none() {
    let locations = Locations(vec![None]);
    let iter = locations.iter();
}

#[test]
fn test_iter_single_slot_some() {
    let locations = Locations(vec![Some(0)]);
    let iter = locations.iter();
}

#[test]
fn test_iter_multiple_slots() {
    let locations = Locations(vec![Some(0), None, Some(5), Some(10)]);
    let iter = locations.iter();
}

#[test]
fn test_iter_max_capacity_slots() {
    let max_capacity = std::usize::MAX;
    let slots = (0..max_capacity).map(|x| Some(x)).collect::<Vec<_>>();
    let locations = Locations(slots);
    let iter = locations.iter();
}

#[test]
fn test_iter_large_slots_with_none() {
    let locations = Locations(vec![Some(0), None, Some(2), None, Some(4)]);
    let iter = locations.iter();
}

