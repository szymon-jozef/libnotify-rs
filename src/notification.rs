include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum Urgency {
    Low,
    Normal,
    Critical,
}

pub enum ClosedReason {
    Unset,
    Expired,
    ApiRequest,
    Undefined,
}

/// Enum representing `[GVariant](https://docs.gtk.org/glib/struct.Variant.html)`
/// Used in set hint.
pub enum HintValue {
    String(String),
    Int32(i32),
    Boolean(bool),
}

pub enum Timeout {
    Default,
    Never,
    Custom(i32),
}

pub struct Notification {
    inner: *mut NotifyNotification,
}

impl<'a> Notification {
    pub fn new<T, Y>(summary: &str, body: T, icon: Y) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: Into<Option<&'a str>>,
        Y: Into<Option<&'a str>>,
    {
        let summary = std::ffi::CString::new(summary)?;

        let body = body
            .into()
            .map(|body| std::ffi::CString::new(body))
            .transpose()?;

        let icon = icon
            .into()
            .map(|icon| std::ffi::CString::new(icon))
            .transpose()?;

        let inner = unsafe {
            notify_notification_new(
                summary.as_ptr(),
                body.as_ref().map_or(std::ptr::null(), |body| body.as_ptr()),
                icon.as_ref().map_or(std::ptr::null(), |icon| icon.as_ptr()),
            )
        };

        if inner.is_null() {
            return Err("notify_notification_new returned null".into());
        }

        Ok(Self { inner })
    }

    pub fn add_action<F>(
        &mut self,
        action: &str,
        label: &str,
        callback: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&str) + 'static,
    {
        let action = std::ffi::CString::new(action)?;
        let label = std::ffi::CString::new(label)?;

        let callback: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(callback) as *mut std::ffi::c_void;

        let free_func = drop_box::<F>;

        unsafe {
            notify_notification_add_action(
                self.inner,
                action.as_ptr(),
                label.as_ptr(),
                Some(action_trampoline::<F>),
                user_data,
                Some(free_func),
            );
        }

        Ok(())
    }

    /// Clears all actions from the notification
    pub fn clear_actions(&mut self) {
        unsafe { notify_notification_clear_actions(self.inner) }
    }

    /// Clears all hints from the notification
    pub fn clear_hints(&mut self) {
        unsafe { notify_notification_clear_hints(self.inner) }
    }

    /// Synchronously tells the notification server to hide the notification on the screen
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut gerror = std::ptr::null_mut() as *mut GError;

        if unsafe { notify_notification_close(self.inner, &mut gerror) } == 0 {
            if gerror.is_null() {
                return Err("Unknown error".into());
            }

            let e = unsafe { *gerror }.message;

            if e.is_null() {
                unsafe { g_error_free(gerror) };
                return Err("Unknown error".into());
            }

            let e_str = unsafe { std::ffi::CStr::from_ptr(e) }
                .to_string_lossy()
                .to_string();

            unsafe { g_error_free(gerror) };

            return Err(e_str.into());
        }

        Ok(())
    }

    /// Returns the closed reason code for the notification.
    ///
    /// This is valid only after the Notification::closed signal is emitted.
    pub fn get_closed_reason(&self) -> ClosedReason {
        todo!();
    }

    /// Sets the application icon for the notification.
    ///
    /// If this function is not called, the application icon will be set from the value set via
    /// set_app_icon().
    ///
    /// Available since: 0.8.4
    pub fn set_app_icon(&mut self, app_icon: &str) {
        todo!();
    }

    /// Sets the application name for the notification.
    ///
    /// If this function is not called, the application name will be set from the value used in init() or overridden with set_app_name().
    pub fn set_app_name(&mut self, app_name: &str) {
        todo!();
    }

    /// Sets the category of this notification.
    ///
    /// This can be used by the notification server to filter or display the data in a certain way
    pub fn set_category(&mut self, category: &str) {
        todo!();
    }

    /// Sets a hint for key with value value
    ///
    /// Available since: 0.6
    pub fn set_hint(&mut self, key: &str, value: HintValue) {
        todo!();
    }

    /// Sets the timeout of the notification.
    ///
    /// Note that the timeout may be ignored by the server.
    pub fn set_timeout(&mut self, timeout: Timeout) {
        todo!();
    }

    /// Sets the urgency level of this notification
    pub fn set_urgency(&mut self, urgency: Urgency) {
        todo!();
    }

    /// Tells the notification server to display the notification on the screen
    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    /// Updates the notification text and icon.
    ///
    /// This won’t send the update out and display it on the screen. For that, you will need to call `show()`.
    pub fn update<'b, F, I>(&mut self, summary: &str, body: F, icon: I)
    where
        F: Into<Option<&'b str>>,
        I: Into<Option<&'b str>>,
    {
        todo!();
    }
}

unsafe extern "C" fn action_trampoline<F>(
    _notification: *mut NotifyNotification,
    action: *mut ::std::os::raw::c_char,
    user_data: gpointer,
) where
    F: Fn(&str) + 'static,
{
    let callback = unsafe { &*(user_data as *const F) };

    if action.is_null() {
        return;
    }

    let action = unsafe { std::ffi::CStr::from_ptr(action).to_string_lossy() };
    callback(&action);
}

unsafe extern "C" fn drop_box<F>(data: gpointer) {
    unsafe { drop(Box::from_raw(data as *mut F)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        assert!(
            !Notification::new("Test", "Test", "Test")
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", None, None)
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", None, "Test")
                .unwrap()
                .inner
                .is_null()
        );
        assert!(
            !Notification::new("Test", "Test", None)
                .unwrap()
                .inner
                .is_null()
        );
    }
}
