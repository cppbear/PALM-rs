fn reader(self) -> Reader<Self>
    where
        Self: Sized,
    {
        reader::new(self)
    }