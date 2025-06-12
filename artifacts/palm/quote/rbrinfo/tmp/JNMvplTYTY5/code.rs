fn span(&self) -> Option<Span> {
        <T as IdentFragment>::span(*self)
    }