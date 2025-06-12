use crate::de::{Deserialize, DeserializeSeed, Deserializer};

/// A DeserializeSeed helper for implementing deserialize_in_place Visitors.
///
/// Wraps a mutable reference and calls deserialize_in_place on it.
pub struct InPlaceSeed<'a, T: 'a>(pub &'a mut T);

impl<'a, 'de, T> DeserializeSeed<'de> for InPlaceSeed<'a, T>
where
    T: Deserialize<'de>,
{
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_in_place(deserializer, self.0)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut i32_0: i32 = -4774i32;
    let mut str_0: &str = "6suhvJRMTzLj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 9932usize;
    let mut i64_0: i64 = 9268i64;
    let mut unexpected_0: de::Unexpected = crate::de::Unexpected::Signed(i64_0);
    let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
    let mut u64_0: u64 = 3879u64;
    let mut unexpected_1: de::Unexpected = crate::de::Unexpected::Unsigned(u64_0);
    let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
    let mut usize_1: usize = 1746usize;
    let mut u32_0: u32 = 2971u32;
    let mut mapaccessdeserializer_0: crate::de::value::MapAccessDeserializer<u32> = crate::de::value::MapAccessDeserializer::new(u32_0);
    let mut mapaccessdeserializer_0_ref_0: &crate::de::value::MapAccessDeserializer<u32> = &mut mapaccessdeserializer_0;
    let mut usize_2: usize = 5301usize;
    let mut u128_0: u128 = 5430u128;
    let mut u32_1: u32 = 9896u32;
    let mut isize_0: isize = -6919isize;
    let mut tuple_0: (isize, crate::de::value::private::UnitOnly<isize>) = crate::de::value::private::unit_only(isize_0);
    let mut content_0: __private::de::content::Content = crate::__private::de::content::Content::U32(u32_1);
    let mut content_0_ref_0: &__private::de::content::Content = &mut content_0;
    let mut option_0: std::option::Option<&str> = crate::__private::de::content::Content::as_str(content_0_ref_0);
    let mut str_1: &str = std::option::Option::unwrap(option_0);
    let mut u128deserializer_0: crate::de::value::U128Deserializer<isize> = crate::de::value::U128Deserializer::new(u128_0);
    let mut mapaccessdeserializer_1: crate::de::value::MapAccessDeserializer<u32> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_0_ref_0);
    let mut u128deserializer_0_ref_0: &crate::de::value::U128Deserializer<isize> = &mut u128deserializer_0;
    let mut u128deserializer_1: crate::de::value::U128Deserializer<isize> = crate::de::value::U128Deserializer::clone(u128deserializer_0_ref_0);
    let mut u128deserializer_1_ref_0: &crate::de::value::U128Deserializer<isize> = &mut u128deserializer_1;
    let mut bool_0: bool = crate::de::Unexpected::eq(unexpected_1_ref_0, unexpected_0_ref_0);
    let mut mapaccessdeserializer_1_ref_0: &crate::de::value::MapAccessDeserializer<u32> = &mut mapaccessdeserializer_1;
    let mut mapaccessdeserializer_2: crate::de::value::MapAccessDeserializer<u32> = crate::de::value::MapAccessDeserializer::clone(mapaccessdeserializer_1_ref_0);
    panic!("From RustyUnit with love");
}
}