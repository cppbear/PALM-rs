pub const fn new(init: F) -> Lazy<T, F> {
            Lazy { cell: OnceCell::new(), init: Cell::new(Some(init)) }
        }