pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    rng().random()
}