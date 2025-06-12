pub fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>) {
        (
            t,
            UnitOnly {
                marker: PhantomData,
            },
        )
    }