pub fn new(slice: &'a [T]) -> Result<Self, Empty> {
        let num_choices = NonZeroUsize::new(slice.len()).ok_or(Empty)?;

        Ok(Self {
            slice,
            range: UniformUsize::new(0, num_choices.get()).unwrap(),
            num_choices,
        })
    }