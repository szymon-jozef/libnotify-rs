use crate::api::{
    errors::LibnotifyError,
    functions::{init, is_initted, set_app_icon, set_app_name, uninit},
};

/// Notification context. Handles libnotify initializing and uninitializing.
pub struct LibnotifyContext {}

static IS_CONTEXT_CREATED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

impl LibnotifyContext {
    /// Try initializing libnotify
    ///
    /// # Error
    /// Returns an error, if libnotify cannot be initialized or if there already exists a context
    /// object
    ///
    /// # Example
    /// ```
    /// use libnotify_rs::context::LibnotifyContext;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ctx = LibnotifyContext::new("Coffee addicted reminder")?;
    ///
    ///     // you don't need to uninitialize libnotify, cause context already does that!
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(app_name: &str) -> Result<LibnotifyContext, Box<dyn std::error::Error>> {
        if IS_CONTEXT_CREATED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Err("You can create only one context at a time".into());
        }

        if !is_initted() {
            init(app_name)?;
        }

        Ok(Self {})
    }

    /// Sets global app name
    pub fn set_new_name(&self, app_name: &str) -> Result<(), LibnotifyError> {
        set_app_name(app_name)?;
        Ok(())
    }

    /// Sets global app icon
    pub fn set_app_icon(&self, app_icon: &str) -> Result<(), LibnotifyError> {
        set_app_icon(app_icon)?;
        Ok(())
    }
}

impl Drop for LibnotifyContext {
    fn drop(&mut self) {
        if is_initted() {
            uninit();

            IS_CONTEXT_CREATED.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
