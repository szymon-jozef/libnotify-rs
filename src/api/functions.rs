use crate::api::errors::LibnotifyError;

use super::utils::c_str_to_rs_str_and_free;

use super::libnotify::*;

/* Initting
 *
 *(what a cool section comment)
*/

/// Initializes libnotify. This must be called before any other functions.
///
/// Starting from 0.8, if the provided app_name is NULL, libnotify will try to figure it out from the running application. Before it was not allowed, and was causing libnotify not to be initialized.
///
/// # Example
///
/// ```
/// use libnotify_rs::api::{
///     functions::init,
///     notification:: Notification,
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init("Test app")?; // we need to init libnotify __before__ any other libnotify code
///     let notification = Notification::new("Test", None, None)?; // Some libnotify call
///     Ok(())
/// }
///
/// ```
///
/// # Errors
/// This function can return LibnotifyError::{InitError, NulError}
pub fn init<'a, T>(app_name: T) -> Result<(), LibnotifyError>
where
    T: Into<Option<&'a str>>,
{
    let app_name = app_name.into().map(std::ffi::CString::new).transpose()?;

    if unsafe {
        notify_init(
            app_name
                .as_ref()
                .map_or(std::ptr::null(), |app_name| app_name.as_ptr()),
        )
    } == 0
    {
        return Err(LibnotifyError::InitError);
    }

    Ok(())
}

/// Uninitializes libnotify.
///
/// This should be called when the program no longer needs libnotify for the rest of its lifecycle, typically just before exitting.
///
/// # Example
/// ```
/// use libnotify_rs::api::functions::{init, uninit};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init("Test app")?;
///     // let notify = ...
///     uninit(); // __always__ uninit libnotify after use
///     Ok(())
/// }
///
/// ```
pub fn uninit() {
    unsafe {
        notify_uninit();
    }
}

/// Gets whether or not libnotify is initialized.
///
/// # Example
/// ```
/// use libnotify_rs::api::functions::{init, is_initted};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     if !is_initted() { // useful for conditional initing
///         init("Test app")?;
///     }
///
///     Ok(())
/// }
///
/// ```
pub fn is_initted() -> bool {
    unsafe { notify_is_initted() != 0 }
}

/* Getters
 *
*/

/// Gets the application name registered.
pub fn get_app_name() -> Option<String> {
    let app_name_c: *const std::os::raw::c_char = unsafe { notify_get_app_name() };

    if app_name_c.is_null() {
        None
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(app_name_c) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

/// Gets the application icon registered.
/// Available since: 0.8.4
pub fn get_app_icon() -> Option<String> {
    let app_icon_c: *const std::os::raw::c_char = unsafe { notify_get_app_icon() };

    if app_icon_c.is_null() {
        None
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

/// Contains server information. Refer to [libnotify get_server_info docs](https://gnome.pages.gitlab.gnome.org/libnotify/func.get_server_info.html) to better understand specific fields
pub struct ServerInfo {
    /// A location to store the server name, or `None`
    pub name: Option<String>,
    /// A location to store the server vendor, or `None`
    pub vendor: Option<String>,
    /// A location to store the server version, or `None`
    pub version: Option<String>,
    /// A location to store the version the service is compliant with, or `None`
    pub spec_version: Option<String>,
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

/* Setters
 *
*/

/// Sets the application name
///
/// # Errors
/// This function can return LibnotifyError::NulError
pub fn set_app_name(app_name: &str) -> Result<(), LibnotifyError> {
    let app_name_c = std::ffi::CString::new(app_name)?;
    unsafe { notify_set_app_name(app_name_c.as_ptr()) };

    Ok(())
}

/// Sets the application icon.
/// Available since: 0.8.4
///
/// app_icon is icon name or path to an image
///
/// # Errors
/// This function can return LibnotifyError::NulError
pub fn set_app_icon(app_icon: &str) -> Result<(), LibnotifyError> {
    let app_icon_c = std::ffi::CString::new(app_icon)?;
    unsafe { notify_set_app_icon(app_icon_c.as_ptr()) };

    Ok(())
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;

    #[test]
    fn test_initializing() {
        assert!(!is_initted());
        assert!(init("Morbius").is_ok());
        assert!(is_initted());
        uninit();
        assert!(!is_initted());
    }

    #[test]
    fn test_bad_init() {
        assert!(matches!(
            init("Mor\0ius").unwrap_err(),
            LibnotifyError::NulError(_)
        ));
    }

    #[test]
    fn test_app_name_set_and_get() {
        let first_name: &str = "Morbius";
        let _ = init(first_name);
        assert_eq!(get_app_name().unwrap(), first_name);

        let new_name: &str = "Milo";
        let _ = set_app_name(new_name);
        assert_eq!(get_app_name().unwrap(), new_name);

        uninit();
    }

    #[test]
    fn test_app_name_bad_name() {
        let _ = init("Morbius");

        let bad_name: &str = "mor\0ius";
        assert!(set_app_name(bad_name).is_err());
        uninit();
    }
}
