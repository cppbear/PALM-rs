pub fn re<'b>(&'b mut self) -> UnwrapMut<'b, R>
    where
        'r: 'b,
    {
        UnwrapMut(self.0)
    }