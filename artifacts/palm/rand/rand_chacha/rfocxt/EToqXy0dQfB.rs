use ppv_lite86::{dispatch, dispatch_light128};
pub use ppv_lite86::Machine;
use ppv_lite86::{
    vec128_storage, ArithOps, BitOps32, LaneWords4, MultiLane, StoreBytes, Vec4, Vec4Ext,
    Vector,
};
pub(crate) const BLOCK: usize = 16;
pub(crate) const BLOCK64: u64 = BLOCK as u64;
const LOG2_BUFBLOCKS: u64 = 2;
const BUFBLOCKS: u64 = 1 << LOG2_BUFBLOCKS;
pub(crate) const BUFSZ64: u64 = BLOCK64 * BUFBLOCKS;
pub(crate) const BUFSZ: usize = BUFSZ64 as usize;
const STREAM_PARAM_NONCE: u32 = 1;
const STREAM_PARAM_BLOCK: u32 = 0;
dispatch!(
    m, Mach, { fn refill_wide(state : & mut ChaCha, drounds : u32, out : & mut [u32;
    BUFSZ]) { refill_wide_impl(m, state, drounds, out); } }
);
dispatch!(
    m, Mach, { fn refill_narrow_rounds(state : & mut ChaCha, drounds : u32) -> State <
    vec128_storage > { let k : Mach::u32x4 = m.vec([0x6170_7865, 0x3320_646e,
    0x7962_2d32, 0x6b20_6574]); let mut x = State { a : k, b : m.unpack(state.b), c : m
    .unpack(state.c), d : m.unpack(state.d), }; for _ in 0..drounds { x = round(x); x =
    undiagonalize(round(diagonalize(x))); } State { a : x.a.into(), b : x.b.into(), c : x
    .c.into(), d : x.d.into(), } } }
);
dispatch_light128!(
    m, Mach, { fn set_stream_param(state : & mut ChaCha, param : u32, value : u64) { let
    d : Mach::u32x4 = m.unpack(state.d); state.d = d.insert((value >> 32) as u32, (param
    << 1) | 1).insert(value as u32, param << 1).into(); } }
);
dispatch_light128!(
    m, Mach, { fn get_stream_param(state : & ChaCha, param : u32) -> u64 { let d :
    Mach::u32x4 = m.unpack(state.d); ((d.extract((param << 1) | 1) as u64) << 32) | d
    .extract(param << 1) as u64 } }
);
dispatch_light128!(
    m, Mach, { fn get_seed(state : & ChaCha) -> [u8; 32] { let b : Mach::u32x4 = m
    .unpack(state.b); let c : Mach::u32x4 = m.unpack(state.c); let mut key = [0u8; 32]; b
    .write_le(& mut key[..16]); c.write_le(& mut key[16..]); key } }
);
dispatch_light128!(
    m, Mach, { fn init_chacha(key : & [u8; 32], nonce : & [u8]) -> ChaCha { let ctr_nonce
    = [0, if nonce.len() == 12 { read_u32le(& nonce[0..4]) } else { 0 }, read_u32le(&
    nonce[nonce.len() - 8..nonce.len() - 4]), read_u32le(& nonce[nonce.len() - 4..]),];
    let key0 : Mach::u32x4 = m.read_le(& key[..16]); let key1 : Mach::u32x4 = m.read_le(&
    key[16..]); ChaCha { b : key0.into(), c : key1.into(), d : ctr_nonce.into(), } } }
);
dispatch_light128!(
    m, Mach, { fn init_chacha_x(key : & [u8; 32], nonce : & [u8; 24], rounds : u32) ->
    ChaCha { let key0 : Mach::u32x4 = m.read_le(& key[..16]); let key1 : Mach::u32x4 = m
    .read_le(& key[16..]); let nonce0 : Mach::u32x4 = m.read_le(& nonce[..16]); let mut
    state = ChaCha { b : key0.into(), c : key1.into(), d : nonce0.into(), }; let x =
    refill_narrow_rounds(& mut state, rounds); let ctr_nonce1 = [0, 0, read_u32le(&
    nonce[16..20]), read_u32le(& nonce[20..24])]; state.b = x.a; state.c = x.d; state.d =
    ctr_nonce1.into(); state } }
);
#[derive(Clone, PartialEq, Eq)]
pub struct ChaCha {
    pub(crate) b: vec128_storage,
    pub(crate) c: vec128_storage,
    pub(crate) d: vec128_storage,
}
impl ChaCha {
    #[inline(always)]
    pub fn new(key: &[u8; 32], nonce: &[u8]) -> Self {
        init_chacha(key, nonce)
    }
    #[inline(always)]
    pub fn refill4(&mut self, drounds: u32, out: &mut [u32; BUFSZ]) {}
    #[inline(always)]
    pub fn set_block_pos(&mut self, value: u64) {}
    #[inline(always)]
    pub fn get_block_pos(&self) -> u64 {}
    #[inline(always)]
    pub fn set_nonce(&mut self, value: u64) {}
    #[inline(always)]
    pub fn get_nonce(&self) -> u64 {
        get_stream_param(self, STREAM_PARAM_NONCE)
    }
    #[inline(always)]
    pub fn get_seed(&self) -> [u8; 32] {}
}
