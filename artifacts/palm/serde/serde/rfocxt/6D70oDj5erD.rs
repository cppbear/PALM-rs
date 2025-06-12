use crate::lib::*;
use crate::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
pub const FIELDS: &[&str] = &["end"];
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Deserialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        *place = tri!(Deserialize::deserialize(deserializer));
        Ok(())
    }
}
enum Field {
    End,
}
impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`end`")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    "end" => Ok(Field::End),
                    _ => Err(Error::unknown_field(value, FIELDS)),
                }
            }
            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    b"end" => Ok(Field::End),
                    _ => {
                        let value = crate::__private::from_utf8_lossy(value);
                        Err(Error::unknown_field(&*value, FIELDS))
                    }
                }
            }
        }
        deserializer.deserialize_identifier(FieldVisitor)
    }
}
