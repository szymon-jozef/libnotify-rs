include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/* Initting
 *
 *(what a cool section comment)
*/

/// Initialized libnotify. This must be called before any other functions.
///
/// Starting from 0.8, if the provided app_name is NULL, libnotify will try to figure it out from the running application. Before it was not allowed, and was causing libnotify not to be initialized.
///
/// # Returns
/// TRUE if successful, or FALSE on error
pub fn init<'a, T>(app_name: T) -> bool
where
    T: Into<Option<&'a str>>,
{
    match app_name
        .into()
        .map(|app_name| std::ffi::CString::new(app_name))
        .transpose()
    {
        Ok(app_name) => unsafe {
            notify_init(
                app_name
                    .as_ref()
                    .map_or(std::ptr::null(), |app_name| app_name.as_ptr()),
            ) != 0
        },

        Err(_) => return false,
    }
}

/// Uninitializes libnotify.
/// This should be called when the program no longer needs libnotify for the rest of its lifecycle, typically just before exitting.
pub fn uninit() {
    unsafe {
        notify_uninit();
    }
}

/// Gets whether or not libnotify is initialized.
pub fn is_initted() -> bool {
    unsafe { notify_is_initted() != 0 }
}

/* Getting
 *
*/

// Gets the application icon registered.
// Available since: 0.8.4
pub fn get_app_icon() -> Option<String> {
    let app_icon_c: *const std::os::raw::c_char = unsafe { notify_get_app_icon() };

    if app_icon_c.is_null() {
        return None;
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(app_icon_c) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

/// Queries the server capabilities.
/// Synchronously queries the server for its capabilities and returns them in a list.
pub fn get_server_caps() -> Option<Vec<String>> {
    let mut vec: Vec<String> = vec![];

    let list = unsafe { notify_get_server_caps() };
    if list.is_null() {
        return None;
    }

    let mut current = list;

    loop {
        let data = unsafe { *current }.data;
        let data_char = data as *const std::os::raw::c_char;

        if !data.is_null() {
            let s: String = unsafe { std::ffi::CStr::from_ptr(data_char) }
                .to_string_lossy()
                .to_string();
            vec.push(s);
        }

        unsafe { g_free(data) };

        if unsafe { *current }.next.is_null() {
            break;
        }

        current = unsafe { *current }.next;
    }

    unsafe { g_list_free(list) };

    if vec.is_empty() { None } else { Some(vec) }
}

pub struct ServerInfo {
    /// A location to store the server name, or `None`
    name: Option<String>,
    /// A location to store the server vendor, or `None`
    vendor: Option<String>,
    /// A location to store the server version, or `None`
    version: Option<String>,
    /// A location to store the version the service is compliant with, or `None`
    spec_version: Option<String>,
}

/// Queries the server for information.
/// Synchronously queries the server for its information, specifically, the name, vendor, server version, and the version of the notifications specification that it is compliant with.
pub fn get_server_info() -> Option<ServerInfo> {
    let mut ret_name = std::ptr::null_mut() as *mut std::os::raw::c_char;
    let mut ret_vendor = std::ptr::null_mut() as *mut std::os::raw::c_char;
    let mut ret_version = std::ptr::null_mut() as *mut std::os::raw::c_char;
    let mut ret_spec_version = std::ptr::null_mut() as *mut std::os::raw::c_char;

    if unsafe {
        notify_get_server_info(
            &mut ret_name,
            &mut ret_vendor,
            &mut ret_version,
            &mut ret_spec_version,
        )
    } == 0
    {
        return None;
    }

    Some(ServerInfo {
        name: unsafe { c_str_to_rs_str_and_free(ret_name) },
        vendor: unsafe { c_str_to_rs_str_and_free(ret_vendor) },
        version: unsafe { c_str_to_rs_str_and_free(ret_version) },
        spec_version: unsafe { c_str_to_rs_str_and_free(ret_spec_version) },
    })
}

unsafe fn c_str_to_rs_str_and_free(c_str: *mut std::os::raw::c_char) -> Option<String> {
    if c_str.is_null() {
        return None;
    }

    let normal_string: String = unsafe { std::ffi::CStr::from_ptr(c_str) }
        .to_string_lossy()
        .to_string();

    unsafe { g_free(c_str as *mut std::ffi::c_void) };

    Some(normal_string)
}

/* Setters
 *
*/

/// Sets the application name
pub fn set_app_name(app_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app_name_c = std::ffi::CString::new(app_name)?;
    unsafe { notify_set_app_name(app_name_c.as_ptr()) };

    Ok(())
}

/// Sets the application icon.
/// Available since: 0.8.4
pub fn set_app_icon(app_icon: &str) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initializing() {
        assert!(!is_initted());
        assert!(init("Morbius"));
        assert!(is_initted());
        uninit();
        assert!(!is_initted());
    }
}
