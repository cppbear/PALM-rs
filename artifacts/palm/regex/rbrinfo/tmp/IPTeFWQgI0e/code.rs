fn no_expansion(&mut self) -> Option<Cow<str>> {
        match memchr(b'$', self.as_bytes()) {
            Some(_) => None,
            None => Some(Cow::Borrowed(*self)),
        }
    }