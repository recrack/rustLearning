use std;
import libc::c_uint;

extern mod crypto {
    fn SHA1(src: *u8, sz: c_uint, out: *u8) -> *u8;
}

fn as_hex(data: ~[u8]) -> ~str {
    let mut acc = ~"";
    for data.each |byte| { acc += fmt!("%02x", byte as uint); }
    return acc;
}

fn sha1(data: ~str) -> ~str unsafe {
    let bytes = str::to_bytes(data);
    let hash = crypto::SHA1(vec::unsafe::to_ptr(bytes),
                            vec::len(bytes) as c_uint, ptr::null());
    return as_hex(vec::unsafe::from_buf(hash, 20u));
}

fn main(args: ~[~str]) {
    io::println(sha1(args[1]));
}
