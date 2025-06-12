pub fn has_subexprs(&self) -> bool {
        match *self {
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Anchor(_)
            | HirKind::WordBoundary(_) => false,
            HirKind::Group(_)
            | HirKind::Repetition(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => true,
        }
    }