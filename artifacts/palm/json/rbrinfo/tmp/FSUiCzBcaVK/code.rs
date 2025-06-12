pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = tri!(to_vec_pretty(value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}