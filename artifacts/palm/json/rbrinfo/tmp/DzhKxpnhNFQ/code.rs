fn serialize_f64(self, value: f64) -> Result<String> {
        if value.is_finite() {
            Ok(ryu::Buffer::new().format_finite(value).to_owned())
        } else {
            Err(float_key_must_be_finite())
        }
    }