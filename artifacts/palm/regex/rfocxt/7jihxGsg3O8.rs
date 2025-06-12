pub type Slot = Option<usize>;
pub struct Locations(Vec<Slot>);
impl Locations {
    pub fn pos(&self, i: usize) -> Option<(usize, usize)> {}
    pub fn iter(&self) -> SubCapturesPosIter {}
    pub fn len(&self) -> usize {
        self.0.len() / 2
    }
}
