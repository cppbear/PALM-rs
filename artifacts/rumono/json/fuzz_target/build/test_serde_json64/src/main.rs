#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
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

fn _to_isize(data:&[u8], index:usize)->isize {
    _to_i64(data, index) as isize
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i64(data:&[u8], index:usize)->i64 {
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn test_function64(_param0 :isize ,_param1 :serde_json::value::Serializer ,_param2 :&str ,_param3 :u32 ,_param4 :&str) {
    let _local0 = <serde_json::value::Number as std::convert::From::<isize>>::from(_param0);
    let _local1: std::result::Result::<serde_json::Value, serde_json::Error> = serde_json::to_value(_local0);
    let _local2_param4_helper1 = _unwrap_result(_local1);
    let _local2_param4_helper2 = &(_local2_param4_helper1);
    let _: serde_json::Result::<serde_json::Value> = <serde_json::value::Serializer as serde::ser::Serializer>::serialize_newtype_variant(_param1, _param2, _param3, _param4, _local2_param4_helper2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 14 {return;}
        let dynamic_length = (data.len() - 12) / 2;
        let _param0 = _to_isize(data, 0);
        let _param1 = serde_json::value::Serializer{};
        let _param2 = _to_str(data, 12 + 0 * dynamic_length, 12 + 1 * dynamic_length);
        let _param3 = _to_u32(data, 8);
        let _param4 = _to_str(data, 12 + 1 * dynamic_length, data.len());
        test_function64(_param0 ,_param1 ,_param2 ,_param3 ,_param4);
    });
}
