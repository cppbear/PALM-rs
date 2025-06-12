fn alloc_err(self, layout: Layout) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError::AllocError { layout },
            Fallibility::Infallible => handle_alloc_error(layout),
        }
    }