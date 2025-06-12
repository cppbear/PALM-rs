type Bits = u32;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
const BIT_SIZE: usize = 32;
const MAX_SIZE_BYTES: usize = 256 * (1 << 10);
fn usize_to_u32(n: usize) -> u32 {
    if (n as u64) > (::std::u32::MAX as u64) {
        panic!("BUG: {} is too big to fit into u32", n)
    }
    n as u32
}
