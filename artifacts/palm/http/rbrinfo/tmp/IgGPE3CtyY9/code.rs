fn default() -> Self {
        HeaderMap::try_with_capacity(0).expect("zero capacity should never fail")
    }