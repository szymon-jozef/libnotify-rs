include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
