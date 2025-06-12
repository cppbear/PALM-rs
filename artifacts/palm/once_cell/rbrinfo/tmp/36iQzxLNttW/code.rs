fn default() -> Lazy<T> {
            Lazy::new(T::default)
        }