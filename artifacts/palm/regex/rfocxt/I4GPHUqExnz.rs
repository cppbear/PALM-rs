pub type Slot = Option<usize>;
pub struct Locations(Vec<Slot>);
pub struct SubCapturesPosIter<'c> {
    idx: usize,
    locs: &'c Locations,
}
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {}
    pub fn iter(&self) -> SubCapturesPosIter {
        SubCapturesPosIter {
            idx: 0,
            locs: self,
        }
    }
    pub fn len(&self) -> usize {}
}
