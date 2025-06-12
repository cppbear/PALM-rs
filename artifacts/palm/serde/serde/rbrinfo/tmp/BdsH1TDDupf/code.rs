fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 15;
            debug_assert_eq!(MAX_LEN, "101.102.103.104".len());
            let mut buf = [b'.'; MAX_LEN];
            let mut written = format_u8(self.octets()[0], &mut buf);
            for oct in &self.octets()[1..] {
                // Skip over delimiters that we initialized buf with
                written += format_u8(*oct, &mut buf[written + 1..]) + 1;
            }
            // Safety: We've only written ASCII bytes to the buffer, so it is valid UTF-8
            let buf = unsafe { str::from_utf8_unchecked(&buf[..written]) };
            serializer.serialize_str(buf)
        } else {
            self.octets().serialize(serializer)
        }
    }