#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            use std::process;
            process::exit(0);
        }
    }
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn test_function49(_param0 :f64 ,_param1 :usize ,_param2 :serde_json::value::Serializer ,_param3 :&str ,_param4 :u32 ,_param5 :&str) {
    let _local0 = <serde_json::Value as std::convert::From::<f64>>::from(_param0);
    let _local1_param0_helper1 = &(_local0);
    let _local1: &serde_json::Value = <serde_json::Value as std::ops::Index::<usize>>::index(_local1_param0_helper1, _param1);
    let _: serde_json::Result::<serde_json::Value> = <serde_json::value::Serializer as serde::ser::Serializer>::serialize_newtype_variant(_param2, _param3, _param4, _param5, _local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 22 {return;}
        let dynamic_length = (data.len() - 20) / 2;
        let _param0 = _to_f64(data, 0);
        let _param1 = _to_usize(data, 8);
        let _param2 = serde_json::value::Serializer{};
        let _param3 = _to_str(data, 20 + 0 * dynamic_length, 20 + 1 * dynamic_length);
        let _param4 = _to_u32(data, 16);
        let _param5 = _to_str(data, 20 + 1 * dynamic_length, data.len());
        test_function49(_param0 ,_param1 ,_param2 ,_param3 ,_param4 ,_param5);
    });
}
