fn r#gen<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        self.random()
    }