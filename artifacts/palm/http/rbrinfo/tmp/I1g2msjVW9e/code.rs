fn create_authority<B, F>(b: B, f: F) -> Result<Authority, InvalidUri>
where
    B: AsRef<[u8]>,
    F: FnOnce(B) -> Bytes,
{
    let s = b.as_ref();
    let authority_end = Authority::parse_non_empty(s)?;

    if authority_end != s.len() {
        return Err(ErrorKind::InvalidUriChar.into());
    }

    let bytes = f(b);

    Ok(Authority {
        // Safety: the postcondition on parse_non_empty() and the check against
        // s.len() ensure that b is valid UTF-8. The precondition on f ensures
        // that this is carried through to bytes.
        data: unsafe { ByteStr::from_utf8_unchecked(bytes) },
    })
}