fn uninit_u8_array() -> [MaybeUninit<u8>; SCRATCH_BUF_SIZE] {
    let arr = MaybeUninit::<[MaybeUninit<u8>; SCRATCH_BUF_SIZE]>::uninit();
    // Safety: assume_init() is claiming that an array of MaybeUninit<>
    // has been initialized, but MaybeUninit<>'s do not require initialization.
    unsafe { arr.assume_init() }
}