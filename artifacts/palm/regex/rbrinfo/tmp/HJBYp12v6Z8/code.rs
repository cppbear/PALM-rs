fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
        Some(Cow::Borrowed(self.0))
    }