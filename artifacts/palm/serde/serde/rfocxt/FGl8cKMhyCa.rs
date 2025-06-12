use crate::lib::*;
use crate::ser::{Error, Impossible, Serialize, Serializer};
macro_rules! fmt_primitives {
    ($($f:ident : $t:ty,)*) => {
        $(fn $f (self, v : $t) -> fmt::Result { Display::fmt(& v, self) })*
    };
}
pub struct T;
impl Error for fmt::Error {
    fn custom<T: Display>(_msg: T) -> Self {
        fmt::Error
    }
}
