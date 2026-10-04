use crate::api::errors::LibnotifyError;

use super::libnotify::*;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
/// Notification urgency level. Refer to [libnotify docs](https://gnome.pages.gitlab.gnome.org/libnotify/enum.Urgency.html) for more info
pub enum Urgency {
    /// Low urgency. Used for unimportant notifications
    Low,
    /// Normal urgency. Used for most standard notifications
    Normal,
    /// Critical urgency. Used for very important notifications
    Critical,
}

#[derive(Debug, Clone, Copy)]
/// Reason why notification was closed. Refer to [libnotify docs](https://gnome.pages.gitlab.gnome.org/libnotify/enum.ClosedReason.html)
pub enum ClosedReason {
    /// Notification not closed
    Unset,
    /// Timeout has expired
    Expired,
    /// It has been dismissed by the user
    Dismissed,
    /// It has been closed by a call to `notify_notification_close()`
    ApiRequest,
    /// Closed by undefined/reserved reasons
    Undefined,

    /// This variant is returned only if libnotify returns some weird enum value, which should never
    /// happen
    Invalid,
}

/// Enum representing `[GVariant](https://docs.gtk.org/glib/struct.Variant.html)`
/// Used in set hint.
#[derive(Debug, Clone)]
pub enum HintValue {
    #[allow(missing_docs)]
    String(String),
    #[allow(missing_docs)]
    Int32(i32),
    #[allow(missing_docs)]
    Boolean(bool),
}

/// Notification timeout, until close
#[derive(Debug, Clone, Copy)]
pub enum Timeout {
    #[allow(missing_docs)]
    Default,
    #[allow(missing_docs)]
    Never,
    /// Custom timeout in milliseconds
    Custom(i32),
}

/// Represents one notification. Wraps most of [libnotify::notifaction](https://gnome.pages.gitlab.gnome.org/libnotify/class.Notification.html) functions.
pub struct Notification {
    inner: *mut NotifyNotification,
}

impl<'a> Notification {
    /// Create new notification.
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NewNotificationError, NulError}
    ///
    /// # Example
    /// ```
    ///
    /// use libnotify_rs::api::{
    ///     functions::{init, uninit},
    ///     notification::Notification,
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("Test app")?;
    ///     let notification = Notification::new("My notification", "Yap yap", None)?;
    ///     // ...
    ///     uninit();
    ///     Ok(())
    /// }
    /// ```
    pub fn new<T, Y>(summary: &str, body: T, icon: Y) -> Result<Self, LibnotifyError>
    where
        T: Into<Option<&'a str>>,
        Y: Into<Option<&'a str>>,
    {
        let summary = std::ffi::CString::new(summary)?;

        let body = body.into().map(std::ffi::CString::new).transpose()?;

        let icon = icon.into().map(std::ffi::CString::new).transpose()?;

        let inner = unsafe {
            notify_notification_new(
                summary.as_ptr(),
                body.as_ref().map_or(std::ptr::null(), |body| body.as_ptr()),
                icon.as_ref().map_or(std::ptr::null(), |icon| icon.as_ptr()),
            )
        };

        if inner.is_null() {
            return Err(LibnotifyError::NewNotificationError);
        }

        Ok(Self { inner })
    }

    /// Add callback to notification.
    ///
    /// This function needs a [glib mainloop](https://docs.gtk.org/glib/main-loop.html)
    ///
    /// # Args
    /// `action` – action identifier. `default` will work for most use cases. Other text will be
    /// displayed as button names.
    ///
    /// `label` – label for user
    ///
    /// `callback` - any closure that accepts &str as an argument, where &str is action name.
    ///
    /// # Errors
    /// This function can return LibnotifyError::NulError
    ///
    /// # Example
    /// ```no_run(There's no notification deamon on CI and no one to click the notification)
    /// use glib::MainLoop;
    /// use libnotify_rs::{
    ///     api::functions::{init, uninit},
    ///     api::notification::Notification,
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("Coffee reminder")?;
    ///     let mut notify = Notification::new("Drink coffee", "Click me when you're done drinking", None)?;
    ///
    ///     // We need to have  a glib main loop in order to receive callback actions
    ///     let main_loop = MainLoop::new(None, false);
    ///     let loop_clone = main_loop.clone();
    ///
    ///     notify.add_action("default", "I'm drinking!", move |action_name| {
    ///         println!("User drunk his coffee!");
    ///         println!("Action was named: {}", action_name);
    ///         loop_clone.quit();
    ///     })?;
    ///
    ///     notify.show()?;
    ///     main_loop.run();
    ///
    ///     println!("Glib mainloop has ended");
    ///
    ///     uninit();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn add_action<F>(
        &mut self,
        action: &str,
        label: &str,
        callback: F,
    ) -> Result<(), LibnotifyError>
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
    ///
    /// # Example
    /// ```
    /// use libnotify_rs::api::{
    ///     functions::{init, uninit},
    ///     notification::Notification,
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("Test app")?;
    ///     let mut notification = Notification::new("My notification", "Yap yap", None)?;
    ///     notification.add_action("default", "yap", |action| {
    ///         // do stuff
    ///     })?;
    ///
    ///     notification.add_action("click", "yappers", |action| {
    ///         // do some other stuff
    ///     })?;
    ///
    ///     notification.clear_actions(); // now there are no callbacks!
    ///
    ///     uninit();
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn clear_actions(&mut self) {
        unsafe { notify_notification_clear_actions(self.inner) }
    }

    /// Clears all hints from the notification
    pub fn clear_hints(&mut self) {
        unsafe { notify_notification_clear_hints(self.inner) }
    }

    /// Synchronously tells the notification server to hide the notification on the screen
    ///
    /// # Errors
    /// This function can return LibnotifyError::{AllocationError, GerrorError}
    ///
    /// # Example
    /// ```no_run(Not notification deamon on CI)
    /// use libnotify_rs::api::{functions::init, notification::Notification};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("Test app")?;
    ///     let notification = Notification::new("My notification", "Yap yap", None)?;
    ///     notification.show()?;
    ///     std::thread::sleep(std::time::Duration::from_secs(5)); // To do this you can just set duration,
    ///     // but this is an example
    ///     notification.close()?; // After 5 seconds no notification
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn close(&self) -> Result<(), LibnotifyError> {
        let mut gerror = std::ptr::null_mut() as *mut GError;

        if unsafe { notify_notification_close(self.inner, &mut gerror) } == 0 {
            if gerror.is_null() {
                return Err(LibnotifyError::AllocationError);
            }

            let e = unsafe { *gerror }.message;

            if e.is_null() {
                unsafe { g_error_free(gerror) };
                return Err(LibnotifyError::AllocationError);
            }

            let e_str = unsafe { std::ffi::CStr::from_ptr(e) }
                .to_string_lossy()
                .to_string();

            unsafe { g_error_free(gerror) };

            return Err(LibnotifyError::GerrorError(e_str));
        }

        Ok(())
    }

    /// Returns the closed reason code for the notification.
    ///
    /// This is valid only after the Notification::closed signal is emitted.
    ///
    /// This function needs a [glib mainloop](https://docs.gtk.org/glib/main-loop.html)
    ///
    /// # Example
    /// ```no_run(No notificaton daemon on CI)
    /// use glib::MainLoop;
    /// use libnotify_rs::{
    ///     api::functions::{init, uninit},
    ///     api::notification::Notification,
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("Coffee reminder")?;
    ///     let mut notify = Notification::new("Drink coffee", "Click me when you're done drinking", None)?;
    ///
    ///     // We need to have  a glib main loop in order to receive closed_reason
    ///     let main_loop = MainLoop::new(None, false);
    ///     let loop_clone = main_loop.clone();
    ///
    ///     notify.show()?;
    ///     main_loop.run();
    ///
    ///     // Thanks to the main loop we can also read why the notification was closed
    ///     println!(
    ///         "Notification closed, because: {:?}",
    ///         notify.get_closed_reason()
    ///     );
    ///
    ///     Ok(())
    /// }
    ///```
    pub fn get_closed_reason(&self) -> ClosedReason {
        match unsafe { notify_notification_get_closed_reason(self.inner) } {
            NotifyClosedReason_NOTIFY_CLOSED_REASON_UNSET => ClosedReason::Unset,
            NotifyClosedReason_NOTIFY_CLOSED_REASON_EXPIRED => ClosedReason::Expired,
            NotifyClosedReason_NOTIFY_CLOSED_REASON_DISMISSED => ClosedReason::Dismissed,
            NotifyClosedReason_NOTIFY_CLOSED_REASON_API_REQUEST => ClosedReason::ApiRequest,
            NotifyClosedReason_NOTIFY_CLOSED_REASON_UNDEFINED => ClosedReason::Undefined,
            _ => ClosedReason::Invalid,
        }
    }

    /// Sets the application icon for the notification.
    ///
    /// Refer to the [documentation](https://specifications.freedesktop.org/icon-naming/latest/) for
    /// icon names.
    ///
    /// If this function is not called, the application icon will be set from the value set via
    /// `set_app_icon()`.
    ///
    /// Available since: 0.8.4
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_app_icon(&mut self, app_icon: &str) -> Result<(), LibnotifyError> {
        let app_icon_c = std::ffi::CString::new(app_icon)?;
        unsafe { notify_notification_set_app_icon(self.inner, app_icon_c.as_ptr()) };

        Ok(())
    }

    /// Sets the application name for the notification.
    ///
    /// If this function is not called, the application name will be set from the value used in init() or overridden with set_app_name().
    ///
    /// Available since: 0.7.3
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_app_name(&mut self, app_name: &str) -> Result<(), LibnotifyError> {
        let app_name_c = std::ffi::CString::new(app_name)?;
        unsafe { notify_notification_set_app_name(self.inner, app_name_c.as_ptr()) };

        Ok(())
    }

    /// Sets the category of this notification.
    ///
    /// This can be used by the notification server to filter or display the data in a certain way
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn set_category(&mut self, category: &str) -> Result<(), LibnotifyError> {
        let category_c = std::ffi::CString::new(category)?;
        unsafe { notify_notification_set_category(self.inner, category_c.as_ptr()) };

        Ok(())
    }

    /// Sets a hint for key with value value
    ///
    /// Available since: 0.6
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError, AllocationError}
    pub fn set_hint(&mut self, key: &str, value: HintValue) -> Result<(), LibnotifyError> {
        let key_c = std::ffi::CString::new(key)?;

        let value_c = match value {
            HintValue::String(s) => {
                let s = std::ffi::CString::new(s)?;
                unsafe { g_variant_new_string(s.as_ptr()) }
            }

            HintValue::Int32(i) => unsafe { g_variant_new_int32(i) },

            HintValue::Boolean(b) => unsafe { g_variant_new_boolean(b as i32) },
        };

        if value_c.is_null() {
            return Err(LibnotifyError::AllocationError);
        }

        unsafe { notify_notification_set_hint(self.inner, key_c.as_ptr(), value_c) };

        Ok(())
    }

    /// Sets the timeout of the notification.
    ///
    /// Note that the timeout may be ignored by the server.
    pub fn set_timeout(&mut self, timeout: Timeout) {
        let timeout_c = match timeout {
            Timeout::Default => NOTIFY_EXPIRES_DEFAULT,
            Timeout::Never => NOTIFY_EXPIRES_NEVER as i32,
            Timeout::Custom(i) => i,
        };

        unsafe { notify_notification_set_timeout(self.inner, timeout_c) }
    }

    /// Sets the urgency level of this notification
    pub fn set_urgency(&mut self, urgency: Urgency) {
        let urgency: u32 = match urgency {
            Urgency::Low => NotifyUrgency_NOTIFY_URGENCY_LOW,
            Urgency::Normal => NotifyUrgency_NOTIFY_URGENCY_NORMAL,
            Urgency::Critical => NotifyUrgency_NOTIFY_URGENCY_CRITICAL,
        };

        unsafe { notify_notification_set_urgency(self.inner, urgency) }
    }

    /// Tells the notification server to display the notification on the screen
    ///
    /// # Errors
    /// This function can return LibnotifyError::{GerrorError, AllocationError}
    ///
    /// # Example
    /// ```no_run(No notification daemon on CI)
    /// use libnotify_rs::api::{
    ///     functions::{init, uninit},
    ///     notification::{self, Notification},
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     init("I love coffee")?;
    ///     let mut notify = Notification::new(
    ///         "Remember to drink coffe!",
    ///         "Coffee is very important for your mental health!",
    ///         None,
    ///     )?;
    ///
    ///     notify.show()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn show(&self) -> Result<(), LibnotifyError> {
        let mut gerror = std::ptr::null_mut() as *mut GError;

        if unsafe { notify_notification_show(self.inner, &mut gerror) } == 0 {
            if gerror.is_null() {
                return Err(LibnotifyError::AllocationError);
            }

            let err_msg = unsafe { *gerror }.message;

            if err_msg.is_null() {
                unsafe { g_error_free(gerror) };
                return Err(LibnotifyError::AllocationError);
            }

            let err_msg_str = unsafe { std::ffi::CStr::from_ptr(err_msg) }
                .to_string_lossy()
                .to_string();

            unsafe { g_error_free(gerror) };
            return Err(LibnotifyError::GerrorError(err_msg_str));
        }

        Ok(())
    }

    /// Updates the notification text and icon.
    ///
    /// This won’t send the update out and display it on the screen. For that, you will need to call `show()`.
    ///
    /// # Errors
    /// This function can return LibnotifyError::{NulError}
    pub fn update<'b, F, I>(
        &mut self,
        summary: &str,
        body: F,
        icon: I,
    ) -> Result<(), LibnotifyError>
    where
        F: Into<Option<&'b str>>,
        I: Into<Option<&'b str>>,
    {
        let summary = std::ffi::CString::new(summary)?;

        let body = body.into().map(std::ffi::CString::new).transpose()?;

        let icon = icon.into().map(std::ffi::CString::new).transpose()?;

        unsafe {
            notify_notification_update(
                self.inner,
                summary.as_ptr(),
                body.as_ref().map_or(std::ptr::null(), |body| body.as_ptr()),
                icon.as_ref().map_or(std::ptr::null(), |icon| icon.as_ptr()),
            );
        }

        Ok(())
    }
}

impl Drop for Notification {
    fn drop(&mut self) {
        if self.inner.is_null() {
            return; // shouldn't happen, but check just in case
        }

        unsafe { g_object_unref(self.inner as *mut std::os::raw::c_void) };
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
