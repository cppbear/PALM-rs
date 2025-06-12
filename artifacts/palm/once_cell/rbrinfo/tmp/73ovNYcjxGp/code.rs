pub const fn new() -> OnceCell<T> {
            OnceCell(Imp::new())
        }