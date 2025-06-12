fn try_entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<'_, T>, MaxSizeReached> {
            HdrName::from_static(self, move |hdr| map.try_entry2(hdr))
        }