fn unwrap_err(self) -> UnwrapErr<Self>
    where
        Self: Sized,
    {
        UnwrapErr(self)
    }