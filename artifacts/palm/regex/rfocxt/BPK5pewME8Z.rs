pub type Slot = Option<usize>;
pub struct Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    re: R,
    text: &'t R::Text,
    last_end: usize,
    last_match: Option<usize>,
}
impl<'t, R> Matches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    pub fn text(&self) -> &'t R::Text {
        self.text
    }
    pub fn regex(&self) -> &R {}
}
