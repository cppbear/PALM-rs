fn parse_hdr<'a>(
    data: &'a [u8],
    b: &'a mut [MaybeUninit<u8>; SCRATCH_BUF_SIZE],
    table: &[u8; 256],
) -> Result<HdrName<'a>, InvalidHeaderName> {
    match data.len() {
        0 => Err(InvalidHeaderName::new()),
        len @ 1..=SCRATCH_BUF_SIZE => {
            // Read from data into the buffer - transforming using `table` as we go
            data.iter()
                .zip(b.iter_mut())
                .for_each(|(index, out)| *out = MaybeUninit::new(table[*index as usize]));
            // Safety: len bytes of b were just initialized.
            let name: &'a [u8] = unsafe { slice_assume_init(&b[0..len]) };
            match StandardHeader::from_bytes(name) {
                Some(sh) => Ok(sh.into()),
                None => {
                    if name.contains(&0) {
                        Err(InvalidHeaderName::new())
                    } else {
                        Ok(HdrName::custom(name, true))
                    }
                }
            }
        }
        SCRATCH_BUF_OVERFLOW..=super::MAX_HEADER_NAME_LEN => Ok(HdrName::custom(data, false)),
        _ => Err(InvalidHeaderName::new()),
    }
}