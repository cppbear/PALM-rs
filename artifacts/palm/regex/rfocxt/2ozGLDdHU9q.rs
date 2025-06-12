pub type Slot = Option<usize>;
pub struct CaptureMatches<'t, R>(
    Matches<'t, R>,
)
where
    R: RegularExpression,
    R::Text: 't;
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
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSync<'r>>);
pub struct Matches<'r, 't>(re_trait::Matches<'t, ExecNoSyncStr<'r>>);
impl<'t, R> CaptureMatches<'t, R>
where
    R: RegularExpression,
    R::Text: 't,
{
    pub fn text(&self) -> &'t R::Text {
        self.0.text()
    }
    pub fn regex(&self) -> &R {}
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
