fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, TryEntryError> {
            Ok(HdrName::from_bytes(self.as_bytes(), move |hdr| {
                map.try_entry2(hdr)
            })??)
        }