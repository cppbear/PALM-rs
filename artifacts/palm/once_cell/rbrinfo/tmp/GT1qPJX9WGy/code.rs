pub const fn with_value(value: T) -> OnceCell<T> {
            OnceCell(Imp::with_value(value))
        }