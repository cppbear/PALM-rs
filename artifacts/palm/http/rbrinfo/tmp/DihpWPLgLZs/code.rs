pub fn new(src: &[u8]) -> Result<AllocatedExtension, InvalidMethod> {
            let mut data: Vec<u8> = vec![0; src.len()];

            write_checked(src, &mut data)?;

            // Invariant: data is exactly src.len() long and write_checked
            // ensures that the first src.len() bytes of data are valid UTF-8.
            Ok(AllocatedExtension(data.into_boxed_slice()))
        }