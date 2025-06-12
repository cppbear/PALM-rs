fn collect_str<T>(self, value: &T) -> fmt::Result
    where
        T: ?Sized + Display,
    {
        Display::fmt(value, self)
    }