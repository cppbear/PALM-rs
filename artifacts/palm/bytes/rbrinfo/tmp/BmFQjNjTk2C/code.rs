fn writer(self) -> Writer<Self>
    where
        Self: Sized,
    {
        writer::new(self)
    }