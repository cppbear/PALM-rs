pub type Slot = Option<usize>;
pub struct Locations(Vec<Slot>);
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {
        let (s, e) = (i * 2, i * 2 + 1);
        match (self.0.get(s), self.0.get(e)) {
            (Some(&Some(s)), Some(&Some(e))) => Some((s, e)),
            _ => None,
        }
    }
    pub fn iter(&self) -> SubCapturesPosIter {}
    pub fn len(&self) -> usize {}
}
