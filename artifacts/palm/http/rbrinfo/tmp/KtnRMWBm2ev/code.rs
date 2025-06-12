pub(super) fn len(&self) -> usize {
        match *self {
            Protocol::Http => 4,
            Protocol::Https => 5,
        }
    }