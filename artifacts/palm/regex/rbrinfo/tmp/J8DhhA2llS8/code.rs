fn push_inst_ptr(data: &mut Vec<u8>, prev: &mut InstPtr, ip: InstPtr) {
    let delta = (ip as i32) - (*prev as i32);
    write_vari32(data, delta);
    *prev = ip;
}