use alloc::vec::{self, Vec};
pub use crate::map::IndexMap;
pub use crate::set::IndexSet;
pub use equivalent::Equivalent;
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
impl HashValue {
    #[inline(always)]
    fn get(self) -> u64 {
        self.0 as u64
    }
}
