pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
            let mut data: [u8; InlineExtension::MAX] = Default::default();

            write_checked(src, &mut data)?;

            // Invariant: write_checked ensures that the first src.len() bytes
            // of data are valid UTF-8.
            Ok(InlineExtension(data, src.len() as u8))
        }