fn sample_iter<T, D>(self, distr: D) -> distr::Iter<D, Self, T>
    where
        D: Distribution<T>,
        Self: Sized,
    {
        distr.sample_iter(self)
    }