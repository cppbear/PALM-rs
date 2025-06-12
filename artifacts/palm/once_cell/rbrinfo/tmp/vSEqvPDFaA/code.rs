fn from(value: T) -> Self {
            OnceCell::with_value(value)
        }