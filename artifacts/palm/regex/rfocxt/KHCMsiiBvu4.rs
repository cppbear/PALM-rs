type Bits = u32;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
const BIT_SIZE: usize = 32;
const MAX_SIZE_BYTES: usize = 256 * (1 << 10);
pub fn should_exec(num_insts: usize, text_len: usize) -> bool {
    let size = ((num_insts * (text_len + 1) + BIT_SIZE - 1) / BIT_SIZE) * 4;
    size <= MAX_SIZE_BYTES
}
