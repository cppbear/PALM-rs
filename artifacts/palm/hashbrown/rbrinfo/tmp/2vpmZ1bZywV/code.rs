fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }