pub fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned,
{
    from_trait(read::IoRead::new(rdr))
}