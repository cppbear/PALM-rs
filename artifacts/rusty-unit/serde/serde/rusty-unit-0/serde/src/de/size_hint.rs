use crate::lib::*;

pub fn from_bounds<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    helper(iter.size_hint())
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn cautious<Element>(hint: Option<usize>) -> usize {
    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

    if mem::size_of::<Element>() == 0 {
        0
    } else {
        cmp::min(
            hint.unwrap_or(0),
            MAX_PREALLOC_BYTES / mem::size_of::<Element>(),
        )
    }
}

fn helper(bounds: (usize, Option<usize>)) -> Option<usize> {
    match bounds {
        (lower, Some(upper)) if lower == upper => Some(upper),
        _ => None,
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_43() {
//     rusty_monitor::set_test_id(43);
//     let mut unexpected_0: de::Unexpected = crate::de::Unexpected::TupleVariant;
//     let mut unexpected_0_ref_0: &de::Unexpected = &mut unexpected_0;
//     let mut option_0: std::option::Option<usize> = std::option::Option::None;
//     let mut usize_0: usize = 4616usize;
//     let mut tuple_0: (usize, std::option::Option<usize>) = (usize_0, option_0);
//     let mut option_1: std::option::Option<usize> = crate::de::size_hint::helper(tuple_0);
//     let mut u8_0: u8 = 96u8;
//     let mut f64_0: f64 = -1804.842337f64;
//     let mut f64deserializer_0: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_0);
//     let mut f64deserializer_0_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_0;
//     let mut str_0: &str = "8o9p";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut f64_1: f64 = 6195.798607f64;
//     let mut f64deserializer_1: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::new(f64_1);
//     let mut f64deserializer_1_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_1;
//     let mut f64_2: f64 = 9800.297456f64;
//     let mut i128_0: i128 = 8281i128;
//     let mut f64deserializer_2: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_0_ref_0);
//     let mut f64deserializer_2_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_2;
//     let mut f64deserializer_3: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_2_ref_0);
//     let mut unexpected_1: de::Unexpected = crate::de::Unexpected::NewtypeStruct;
//     let mut unexpected_1_ref_0: &de::Unexpected = &mut unexpected_1;
//     let mut f64deserializer_3_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_3;
//     let mut f64deserializer_4: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_3_ref_0);
//     let mut usize_1: usize = crate::de::size_hint::cautious(option_1);
//     let mut unexpected_2: de::Unexpected = crate::de::Unexpected::TupleVariant;
//     let mut unexpected_2_ref_0: &de::Unexpected = &mut unexpected_2;
//     let mut bool_0: bool = crate::de::Unexpected::eq(unexpected_2_ref_0, unexpected_0_ref_0);
//     let mut f64deserializer_4_ref_0: &crate::de::value::F64Deserializer<isize> = &mut f64deserializer_4;
//     let mut f64deserializer_5: crate::de::value::F64Deserializer<isize> = crate::de::value::F64Deserializer::clone(f64deserializer_4_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_49() {
//     rusty_monitor::set_test_id(49);
//     let mut char_0: char = 'U';
//     let mut i128_0: i128 = 5583i128;
//     let mut i128deserializer_0: crate::de::value::I128Deserializer<isize> = crate::de::value::I128Deserializer::new(i128_0);
//     let mut i128deserializer_0_ref_0: &crate::de::value::I128Deserializer<isize> = &mut i128deserializer_0;
//     let mut unitdeserializer_0: crate::de::value::UnitDeserializer<isize> = crate::de::value::UnitDeserializer::new();
//     let mut unitdeserializer_0_ref_0: &crate::de::value::UnitDeserializer<isize> = &mut unitdeserializer_0;
//     let mut u32_0: u32 = 474u32;
//     let mut u32deserializer_0: crate::de::value::U32Deserializer<isize> = crate::de::value::U32Deserializer::new(u32_0);
//     let mut u32deserializer_0_ref_0: &crate::de::value::U32Deserializer<isize> = &mut u32deserializer_0;
//     let mut option_0: std::option::Option<usize> = std::option::Option::None;
//     let mut usize_0: usize = 141usize;
//     let mut tuple_0: (usize, std::option::Option<usize>) = (usize_0, option_0);
//     let mut option_1: std::option::Option<usize> = crate::de::size_hint::helper(tuple_0);
//     let mut str_0: &str = "zFcAtVgDNniYfClo";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut str_1: &str = "nLP";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut u8_0: u8 = 80u8;
//     let mut u8deserializer_0: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::new(u8_0);
//     let mut u8deserializer_0_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_0;
//     let mut u8deserializer_1: crate::de::value::U8Deserializer<isize> = crate::de::value::U8Deserializer::clone(u8deserializer_0_ref_0);
//     let mut content_0: __private::ser::content::Content = crate::__private::ser::content::Content::None;
//     let mut usize_1: usize = crate::de::size_hint::cautious(option_1);
//     let mut contentserializer_0: crate::__private::ser::content::ContentSerializer<isize> = crate::__private::ser::content::ContentSerializer::new();
//     let mut u32deserializer_1: crate::de::value::U32Deserializer<isize> = crate::de::value::U32Deserializer::clone(u32deserializer_0_ref_0);
//     let mut u8deserializer_1_ref_0: &crate::de::value::U8Deserializer<isize> = &mut u8deserializer_1;
//     let mut content_1: __private::ser::content::Content = crate::__private::ser::content::Content::Char(char_0);
//     let mut u32deserializer_1_ref_0: &crate::de::value::U32Deserializer<isize> = &mut u32deserializer_1;
//     let mut u32deserializer_2: crate::de::value::U32Deserializer<isize> = crate::de::value::U32Deserializer::clone(u32deserializer_1_ref_0);
//     let mut u32deserializer_2_ref_0: &crate::de::value::U32Deserializer<isize> = &mut u32deserializer_2;
//     let mut u32deserializer_3: crate::de::value::U32Deserializer<isize> = crate::de::value::U32Deserializer::clone(u32deserializer_2_ref_0);
//     panic!("From RustyUnit with love");
// }

}