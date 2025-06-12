fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn escape_bytes(bytes: &[u8]) -> String {
            let mut s = String::new();
            for &b in bytes {
                s.push_str(&escape_byte(b));
            }
            s
        }

        fn escape_byte(byte: u8) -> String {
            use std::ascii::escape_default;

            let escaped: Vec<u8> = escape_default(byte).collect();
            String::from_utf8_lossy(&escaped).into_owned()
        }

        // We'd like to show something nice here, even if it means an
        // allocation to build a reverse index.
        let slot_to_name: HashMap<&usize, &String> =
            self.0.named_groups.iter().map(|(a, b)| (b, a)).collect();
        let mut map = f.debug_map();
        for (slot, m) in self.0.locs.iter().enumerate() {
            let m = m.map(|(s, e)| escape_bytes(&self.0.text[s..e]));
            if let Some(name) = slot_to_name.get(&slot) {
                map.entry(&name, &m);
            } else {
                map.entry(&slot, &m);
            }
        }
        map.finish()
    }