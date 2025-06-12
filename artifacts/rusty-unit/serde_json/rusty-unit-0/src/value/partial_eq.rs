use super::Value;
use alloc::string::String;

fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64() == Some(other)
}

fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64() == Some(other)
}

fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32() == Some(other),
        _ => false,
    }
}

fn eq_f64(value: &Value, other: f64) -> bool {
    value.as_f64() == Some(other)
}

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool() == Some(other)
}

fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str() == Some(other)
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}

impl PartialEq<Value> for &str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a mut Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f32[f32]
    eq_f64[f64]
    eq_bool[bool]
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::iter::Iterator;
	use std::iter::DoubleEndedIterator;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_22() {
//     rusty_monitor::set_test_id(22);
//     let mut usize_0: usize = 8642usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::into_iter(map_0_ref_0);
//     let mut itermut_0_ref_0: &crate::map::IterMut = &mut itermut_0;
//     let mut bool_0: bool = true;
//     let mut value_0: value::Value = crate::value::Value::Bool(bool_0);
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
//     let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
//     let mut usize_1: usize = 7392usize;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
//     let mut intoiter_1: crate::map::IntoIter = crate::map::Map::into_iter(map_2);
//     let mut intoiter_1_ref_0: &crate::map::IntoIter = &mut intoiter_1;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
//     let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_3_ref_0);
//     let mut iter_0_ref_0: &crate::map::Iter = &mut iter_0;
//     let mut str_0: &str = "bjsRCTfHOkx4DT6Q4";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut f32_0: f32 = 12451.401402f32;
//     let mut bool_1: bool = false;
//     let mut value_1: value::Value = crate::value::Value::Bool(bool_1);
//     let mut value_1_ref_0: &value::Value = &mut value_1;
//     let mut value_2: value::Value = crate::value::Value::Null;
//     let mut bool_2: bool = crate::value::partial_eq::eq_f32(value_1_ref_0, f32_0);
//     let mut deserializer_0: crate::de::Deserializer<crate::read::StrRead> = crate::de::Deserializer::from_str(str_0_ref_0);
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Iter::size_hint(iter_0_ref_0);
//     let mut usize_2: usize = crate::map::IntoIter::len(intoiter_1_ref_0);
//     let mut option_0: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_0_ref_0);
//     let mut usize_3: usize = crate::map::IterMut::len(itermut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_1_ref_0);
    let mut keys_0_ref_0: &crate::map::Keys = &mut keys_0;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 6870usize;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut value_0: value::Value = crate::value::Value::Object(map_2);
    let mut prettyformatter_0: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::default();
    let mut prettyformatter_0_ref_0: &crate::ser::PrettyFormatter = &mut prettyformatter_0;
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
    let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_4_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_4;
    let mut keys_1: crate::map::Keys = crate::map::Map::keys(map_4_ref_0);
    let mut keys_1_ref_0: &crate::map::Keys = &mut keys_1;
    let mut bool_0: bool = false;
    let mut vec_0: std::vec::Vec<value::Value> = std::vec::Vec::new();
    let mut value_1: value::Value = crate::value::Value::Array(vec_0);
    let mut value_1_ref_0: &value::Value = &mut value_1;
    let mut bool_1: bool = crate::value::partial_eq::eq_bool(value_1_ref_0, bool_0);
    let mut usize_1: usize = crate::map::Keys::len(keys_1_ref_0);
    let mut prettyformatter_1: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::clone(prettyformatter_0_ref_0);
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut usize_2: usize = crate::map::Keys::len(keys_0_ref_0);
    let mut keys_2: crate::map::Keys = crate::map::Map::keys(map_0_ref_0);
    let mut state_0: ser::State = crate::ser::State::First;
    panic!("From RustyUnit with love");
}
}