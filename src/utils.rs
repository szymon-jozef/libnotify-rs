include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn c_str_to_rs_str_and_free(c_str: *mut std::os::raw::c_char) -> Option<String> {
    if c_str.is_null() {
        return None;
    }

    let normal_string: String = unsafe { std::ffi::CStr::from_ptr(c_str) }
        .to_string_lossy()
        .to_string();

    unsafe { g_free(c_str as *mut std::ffi::c_void) };

    Some(normal_string)
}
