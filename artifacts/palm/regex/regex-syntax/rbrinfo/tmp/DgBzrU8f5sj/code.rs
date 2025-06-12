pub fn byte(&self) -> Option<u8> {
        let short_hex = LiteralKind::HexFixed(HexLiteralKind::X);
        if self.c as u32 <= 255 && self.kind == short_hex {
            Some(self.c as u8)
        } else {
            None
        }
    }