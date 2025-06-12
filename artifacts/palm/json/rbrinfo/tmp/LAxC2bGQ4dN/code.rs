fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = tri!(de::Deserialize::deserialize(&mut de));

    // Make sure the whole stream has been consumed.
    tri!(de.end());
    Ok(value)
}