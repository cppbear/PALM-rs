pub fn new() -> Self {
        HeaderMap::try_with_capacity(0).unwrap()
    }