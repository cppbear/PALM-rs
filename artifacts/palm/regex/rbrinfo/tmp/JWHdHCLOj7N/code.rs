fn no_expansion(&mut self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.0))
    }