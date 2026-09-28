# FFI

## C Interop
```lp
use std::ffi::{c_int, c_char, CStr, CString}

extern "C" {
    fn puts(s: *const c_char) -> c_int;
}

fn main() {
    let msg = CString::new("Hello from C!").unwrap();
    unsafe { puts(msg.as_ptr()) }
}
```