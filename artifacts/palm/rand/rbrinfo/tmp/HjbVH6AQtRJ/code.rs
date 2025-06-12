pub fn random_iter<T>() -> distr::Iter<StandardUniform, rngs::ThreadRng, T>
where
    StandardUniform: Distribution<T>,
{
    rng().random_iter()
}