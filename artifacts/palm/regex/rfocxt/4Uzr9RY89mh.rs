pub type Slot = Option<usize>;
pub struct Locations(Vec<Slot>);
pub fn as_slots(locs: &mut Locations) -> &mut [Slot] {
    &mut locs.0
}
