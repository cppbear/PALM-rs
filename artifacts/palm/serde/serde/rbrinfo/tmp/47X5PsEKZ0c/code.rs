fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> fmt::Result {
        Display::fmt(variant, self)
    }