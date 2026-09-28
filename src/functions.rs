include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Initialized libnotify. This must be called before any other functions.
///
/// Starting from 0.8, if the provided app_name is NULL, libnotify will try to figure it out from the running application. Before it was not allowed, and was causing libnotify not to be initialized.
///
/// # Returns
/// True if successful, false if libnotify fails to initialize, error if string contains \0
pub fn init<'a, T>(app_name: T) -> Result<bool, Box<dyn std::error::Error>>
where
    T: Into<Option<&'a str>>,
{
    let app_name = app_name
        .into()
        .map(|app_name| std::ffi::CString::new(app_name))
        .transpose()?;

    let result = unsafe {
        notify_init(
            app_name
                .as_ref()
                .map_or(std::ptr::null(), |app_name| app_name.as_ptr()),
        ) != 0
    };

    Ok(result)
}
