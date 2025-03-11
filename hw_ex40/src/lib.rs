use std::ffi::{c_char, c_int};

/// A function that takes a C string and a mutable reference to a `StringParams` struct.
///
/// # Safety
///
/// This function is marked as `unsafe` because it takes a raw pointer to a C string.
/// The caller must ensure that the pointer is valid and points to a null-terminated string.
/// Additionally, the caller must ensure that the `StringParams` struct is properly initialized
/// and that the function is not called concurrently with other functions that access the same
/// `StringParams` struct.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_params(s: *const c_char, params: &mut StringParams) -> ErrorCode {
    let c_s = unsafe { std::ffi::CStr::from_ptr(s) };

    let Ok(s) = c_s.to_str() else {
        return ErrorCode::BadEncoding;
    };
    
    params.len = s.chars().count() as c_int;
    params.capitals = s.chars().filter(|c| c.is_ascii_uppercase()).count() as c_int;
    ErrorCode::Ok
}

#[repr(C)]
pub struct StringParams {
    pub len : c_int,
    pub capitals : c_int,

}

#[repr(u8)]
pub enum ErrorCode {
    Ok,
    BadEncoding
}