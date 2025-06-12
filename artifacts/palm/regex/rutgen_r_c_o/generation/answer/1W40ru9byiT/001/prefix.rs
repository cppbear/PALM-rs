// Answer 0

#[test]
fn test_as_slots_empty_locations() {
    let mut locs = Locations(vec![]);
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_single_none() {
    let mut locs = Locations(vec![None]);
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_single_some_zero() {
    let mut locs = Locations(vec![Some(0)]);
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_single_some_max() {
    let mut locs = Locations(vec![Some(1000)]);
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_multiple_mixed() {
    let mut locs = Locations(vec![None, Some(1), None, Some(500), Some(1000)]);
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_large_count() {
    let mut locs = Locations((0..=1000).map(|i| Some(i)).collect());
    let slots = as_slots(&mut locs);
}

#[test]
fn test_as_slots_large_count_with_none() {
    let mut locs = Locations((0..=1000).map(|i| if i % 2 == 0 { None } else { Some(i) }).collect());
    let slots = as_slots(&mut locs);
}

