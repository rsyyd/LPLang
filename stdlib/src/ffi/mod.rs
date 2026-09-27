// FFI: C bindings

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type c_void = ();

pub struct CStr {
    ptr: *const c_char,
}

impl CStr {
    pub fn from_ptr(ptr: *const c_char) -> &'static Self {
        // TODO
        unimplemented!()
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        Ok("")
    }
}

pub struct CString {
    vec: Vec<u8>,
}

impl CString {
    pub fn new(s: &str) -> Result<Self, NulError> {
        Ok(Self { vec: s.as_bytes().to_vec() })
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.vec.as_ptr() as *const c_char
    }
}

#[derive(Debug)]
pub struct NulError;
#[derive(Debug)]
pub struct Utf8Error;