fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
        match memchr(b'$', *self) {
            Some(_) => None,
            None => Some(Cow::Borrowed(*self)),
        }
    }