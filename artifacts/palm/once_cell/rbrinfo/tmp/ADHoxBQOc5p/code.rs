pub const fn new(f: F) -> Lazy<T, F> {
            Lazy { cell: OnceCell::new(), init: Cell::new(Some(f)) }
        }