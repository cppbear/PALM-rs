fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (HeaderName, T)>,
    {
        let mut map = HeaderMap::default();
        map.extend(iter);
        map
    }